fn main() {
	println!("cargo:rerun-if-env-changed=OPTIVORBIS_VERSION");
	println!("cargo:rerun-if-env-changed=OPTIVORBIS_BUILD_DATE");

	if option_env!("OPTIVORBIS_VERSION").is_none() {
		println!(
			"cargo:rustc-env=OPTIVORBIS_VERSION=v{}{}",
			env!("CARGO_PKG_VERSION"),
			option_env!("CARGO_PRIMARY_PACKAGE")
				.map(|_| "-custom")
				.unwrap_or_default()
		);
	}
	match option_env!("OPTIVORBIS_BUILD_DATE") {
		Some(build_date) => {
			println!("cargo:rustc-env=OPTIVORBIS_BUILD_DATE_VERSION_SUFFIX= ({build_date})");
		}
		None => println!("cargo:rustc-env=OPTIVORBIS_BUILD_DATE_VERSION_SUFFIX=")
	}
}
