# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to
[Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

No changes yet.

## [0.1.2] - 2022-08-14

### Changed
- The Ogg to Ogg remuxer now errors out by default if no Vorbis audio streams
  are found, better matching user expectations, as this usually signals usage
  mistakes. The previous behavior of outputting an empty Ogg file can be
  opted-in via the new `error_on_no_vorbis_streams` remuxer option.
- The build dependency on the `chrono` crate was removed. The build script now
  depends on the `time` crate, which is more lightweight and better maintained.

## [0.1.1] - 2022-07-07
### Added
- Document minimum supported Rust version (MSRV) in the `README.md` file.

### Fixed
- Fix build of the `optivorbis` crate as published in crates.io, by updating a
  dependency that previously required patching its source to be compatible
  with the crate.

## [0.1.0] - 2022-07-06
### Added
- First public release

[Unreleased]: https://github.com/OptiVorbis/OptiVorbis/compare/v0.1.2...HEAD
[0.1.2]: https://github.com/OptiVorbis/OptiVorbis/compare/v0.1.1...v0.1.2
[0.1.1]: https://github.com/OptiVorbis/OptiVorbis/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/OptiVorbis/OptiVorbis/releases/tag/v0.1.0