#!/bin/sh -e
# Runs the American fuzzy lop (AFL) fuzzer, using afl.rs, on the setup AFL
# targets. Documentation: https://rust-fuzz.github.io/book/afl.html
#
# Fuzzing is only available on Unix-like platforms.

readonly FUZZ_OUTPUT_DIR=afl_fuzz

fix_trouble_files_crc() {
	echo "> Fixing test case Ogg files CRC"

	# If available, run rogg_crcfix to recompute any CRC data and allow interesting
	# files to be accepted by non-instrumented code. The source of this program is
	# available at https://gitlab.xiph.org/xiph/rogg
	if command -v rogg_crcfix >/dev/null 2>&1; then
		for file in "$FUZZ_OUTPUT_DIR"/*/crashes/* "$FUZZ_OUTPUT_DIR"/*/hangs/*; do
			rogg_crcfix "$file" || true
		done
	fi
}

cd "$(git rev-parse --show-toplevel)"

for fuzz_target in packages/*_afl_fuzz_target; do
	# The ogg library has a fuzzing feature flag that disables CRC checks,
	# set by cargo afl. See: https://github.com/RustAudio/ogg/pull/6
	cargo afl build --profile fuzzing -p "${fuzz_target#packages/}"
done

# Launch the fuzzing
num_cores=$(nproc 2>/dev/null || echo 1)
secondary_pids=""

cleanup() {
	if [ -n "$secondary_pids" ]; then
		# shellcheck disable=SC2086
		kill $secondary_pids 2>/dev/null || true
	fi
	fix_trouble_files_crc
}
trap 'cleanup' INT QUIT TERM EXIT

for fuzz_target_package in packages/*_afl_fuzz_target; do
	fuzz_target="${fuzz_target_package#packages/}"
	echo "> Fuzzing $fuzz_target with $num_cores core(s)"

	# Start one secondary fuzzer per extra available core
	i=1
	while [ "$i" -lt "$num_cores" ]; do
		AFL_AUTORESUME=1 AFL_SKIP_CPUFREQ=1 AFL_NO_UI=1 \
		cargo afl fuzz -S "secondary_$i" \
			-i packages/optivorbis/resources/test -o "$FUZZ_OUTPUT_DIR" \
			-- "target/fuzzing/$fuzz_target" >/dev/null 2>&1 &
		secondary_pids="$secondary_pids $!"
		i=$((i + 1))
	done

	# Run the main fuzzer in the foreground so AFL's built-in ncurses TUI is
	# visible
	AFL_AUTORESUME=1 AFL_SKIP_CPUFREQ=1 \
	cargo afl fuzz -M main -i packages/optivorbis/resources/test -o "$FUZZ_OUTPUT_DIR" \
		-- "target/fuzzing/$fuzz_target"
done
