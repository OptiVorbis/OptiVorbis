use std::env::current_dir;
use std::error::Error;

use git2::{DescribeFormatOptions, DescribeOptions, Repository};
use time::OffsetDateTime;

fn main() {
	match git_version() {
		Ok(version) => {
			println!("cargo:rustc-env=OPTIVORBIS_VERSION={}", version);
		}
		Err(git_err) => {
			println!(
				"cargo:warning=Could not get version via git: {}. \
				Falling back to Cargo package version",
				git_err
			);
			println!(
				"cargo:rustc-env=OPTIVORBIS_VERSION=v{}",
				env!("CARGO_PKG_VERSION")
			);
		}
	};

	set_build_date_env();
}

fn git_version() -> Result<String, Box<dyn Error>> {
	// The current directory is set to the source file directory for this package.
	// Find the repo directory backtracking on the file tree
	let repo = Repository::discover(current_dir()?)?;

	// Make sure we're executed if HEAD changes
	println!("cargo:rerun-if-changed={:?}", repo.path().join("HEAD"));

	// Run the equivalent of git describe --tags --dirty=-custom --always
	let head_description = repo.describe(
		DescribeOptions::new()
			.describe_tags()
			.show_commit_oid_as_fallback(true)
	)?;

	let version_string =
		head_description.format(Some(DescribeFormatOptions::new().dirty_suffix("-custom")))?;

	Ok(version_string)
}

fn set_build_date_env() {
	// ISO 8601 YYYY-MM-DD date
	let build_date = OffsetDateTime::now_utc();
	let (build_year, build_month, build_day) = build_date.to_calendar_date();
	println!(
		"cargo:rustc-env=OPTIVORBIS_BUILD_DATE={:04}-{:02}-{:02}",
		build_year, build_month as u8, build_day
	);
}
