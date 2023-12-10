# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to
[Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- OptiVorbis now conforms to the [`SOURCE_DATE_EPOCH` specification defined by
  the Reproducible Builds
  project](https://reproducible-builds.org/specs/source-date-epoch/), allowing
  it to generate the same output files for the same set of operations with a
  given set of input files. (Related issue:
  [#41](https://github.com/OptiVorbis/OptiVorbis/issues/41))
  - When the `SOURCE_DATE_EPOCH` environment variable is set, identical files
    processed in different OptiVorbis runs will have identical stream serials,
    which may cause minor interoperability problems. Therefore, it is
    recommended to use this reproducible output mode only when the benefits of
    achieving reproducible output outweigh the costs of non-compliance with the
    Ogg specification.
  - It is possible to switch off support for this specification at compile time
    by disabling the new `source-date-epoch` feature, which is enabled by
    default.

### Changed

- Make `BitpackReader` and `BitpackWriter` read and write one instead of several
  bytes a time, which causes the Rust compiler to not issue costly `memcpy`
  calls for most bitpacked data operations on buffered I/O sources and sinks.
  ([#32](https://github.com/OptiVorbis/OptiVorbis/issues/32#issuecomment-1674076883))
  - This results in a ~15% execution time improvement for the official,
  statically linked OptiVorbis Linux binaries, due to `musl`'s less optimized
  `memcpy` implementation, but it also slightly improves performance for `glibc`
  binaries by ~1%.
  - Unbuffered I/O sources and sinks that make one system call per read or write
    operation will have worse performance, but since the bitpacking code
    documentation already discouraged them due to their fundamentally bad
    performance, this should not be an issue for most applications.
- Added `documentation` metadata to the project workspace packages to make the
  documentation slightly more discoverable on `crates.io`.
- Minor dependency updates.

### Removed

- Remove the redundant `PacketType::from_repr` method to drop the dependency on
  `strum_macros`. Please use the `TryFrom<u8>` trait implementation instead.
- Remove build dependencies on `git2` and `time` in favor of gathering build
  date and version metadata during CI workflows, which greatly reduces build
  times (~42% on a developer workstation).

## [0.1.4] - 2023-06-18

### Changed

- Bump MSRV to 1.65.
- Fix [RUSTSEC-2023-0042](https://rustsec.org/advisories/RUSTSEC-2023-0042) by
  updating `ouroboros`.
- Several minor dependency updates.

## [0.1.3] - 2023-04-03

### Changed

- Bump MSRV to 1.64.
- Improve some documentation wording.
- The author field of the Cargo packages in the repository now includes an
  e-mail address.
- The CI workflow now passes the unstable `-Zdoctest-xcompile` option to run
  doctests when cross-compiling AArch64/ARM64 binaries, increasing test coverage
  for these platforms.
- Refactor the Cargo workspace to leverage property inheritance.
- Several dependency updates. The most important of these is the build-time
  dependency on `git2`, which is necessary to avoid `libgit2-sys` conflicts on
  projects that depend on the latest `git2` version.

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

- First public release.

[Unreleased]: https://github.com/OptiVorbis/OptiVorbis/compare/v0.1.4...HEAD
[0.1.4]: https://github.com/OptiVorbis/OptiVorbis/compare/v0.1.3...v0.1.4
[0.1.3]: https://github.com/OptiVorbis/OptiVorbis/compare/v0.1.2...v0.1.3
[0.1.2]: https://github.com/OptiVorbis/OptiVorbis/compare/v0.1.1...v0.1.2
[0.1.1]: https://github.com/OptiVorbis/OptiVorbis/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/OptiVorbis/OptiVorbis/releases/tag/v0.1.0
