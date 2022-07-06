use std::env::current_dir;
use std::error::Error;
use std::time::{SystemTime, UNIX_EPOCH};

use chrono::{TimeZone, Utc};
use git2::{DescribeFormatOptions, DescribeOptions, Repository};

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
	const DURATION_CAST_ERROR: &str =
		"The current time cannot be represented as a signed 64-bit integer";

	let now = Utc.timestamp(
		SystemTime::now().duration_since(UNIX_EPOCH).map_or_else(
			|err| -i64::try_from(err.duration().as_secs()).expect(DURATION_CAST_ERROR),
			|duration| duration.as_secs().try_into().expect(DURATION_CAST_ERROR)
		),
		0
	);

	// ISO 8601 YYYY-MM-DD date
	println!("cargo:rustc-env=OPTIVORBIS_BUILD_DATE={}", now.format("%F"));
}
