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
		for file in "$FUZZ_OUTPUT_DIR"/default/crashes/* "$FUZZ_OUTPUT_DIR"/default/hangs/*; do
			rogg_crcfix "$file" || true
		done
	fi
}

cd "$(git rev-parse --show-toplevel)"

for fuzz_target in packages/*_afl_fuzz_target; do
	# Pass the fuzzing feature flag to disable CRC checks in the ogg library.
	# See: https://github.com/RustAudio/ogg/pull/6
	RUSTFLAGS='--cfg=fuzzing' cargo afl build --profile fuzzing \
	-p "${fuzz_target#packages/}"
done

# Minimize corpus to whatever generates interesting cases
for fuzz_target_package in packages/*_afl_fuzz_target; do
	fuzz_target="${fuzz_target_package#packages/}"
	echo "> Minimizing corpus for $fuzz_target"

	cargo afl cmin -i packages/optivorbis/resources/test -o afl_fuzz_input -- \
	"target/fuzzing/$fuzz_target"
done

# Launch the fuzzing
trap 'fix_trouble_files_crc' INT QUIT TERM EXIT
for fuzz_target_package in packages/*_afl_fuzz_target; do
	fuzz_target="${fuzz_target_package#packages/}"
	echo "> Fuzzing $fuzz_target"

	AFL_AUTORESUME=1 cargo afl fuzz -i afl_fuzz_input -o "$FUZZ_OUTPUT_DIR" -- \
	"target/fuzzing/$fuzz_target"
done
