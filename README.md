<div align="center">
<img src="https://github.com/OptiVorbis/OptiVorbis/raw/master/web/src/static/optivorbis_logo.png" alt="OptiVorbis logo" width="33%">
<h1>OptiVorbis</h1>

<i>A library and application for lossless, format-preserving, two-pass optimization and repair of Vorbis data, reducing its size without altering any audio information.</i>

<a href="https://github.com/OptiVorbis/OptiVorbis/actions?query=workflow%3ACI"><img alt="CI workflow status"
src="https://github.com/OptiVorbis/OptiVorbis/actions/workflows/ci.yml/badge.svg"></a>
<a href="https://crates.io/crates/optivorbis"><img alt="crates.io latest version" src="https://img.shields.io/crates/v/optivorbis"></a>
<a href="https://docs.rs/optivorbis"><img alt="docs.rs status" src="https://img.shields.io/docsrs/optivorbis?label=docs.rs"></a>

<img alt="crates.io downloads" src="https://img.shields.io/crates/d/optivorbis?label=crates.io%20downloads">
<img alt="GitHub Releases downloads" src="https://img.shields.io/github/downloads/OptiVorbis/OptiVorbis/total?label=GitHub%20Releases%20downloads">
</div>

# 🔍 Overview

OptiVorbis does lossless optimizations and repairs of complete, seekable Vorbis I audio streams, usually contained in Ogg Vorbis (`.ogg`) files, as defined in the [Vorbis I specification](https://xiph.org/vorbis/doc/Vorbis_I_spec.pdf).

The optimization is _lossless_: streams processed by this library are guaranteed to be decoded to the same audio samples as before by any sane decoder. The internal file structure may differ substantially, but these differences are transparent for end-users, as the resulting streams still conform to the specification.

In addition, OptiVorbis' understanding of the Vorbis format and container encapsulations, combined with the somewhat more lenient, purpose-built parsers it uses, provide it with some repair capabilities. It also tends to output more detailed and actionable error information on failure than other tools, rendering it suitable for sanity-checking Vorbis streams.

Currently, OptiVorbis optimizes Ogg Vorbis streams in the following ways, leveraging the great flexibility provided by the Vorbis I specification. Some of these require doing two passes over the entire Vorbis stream:

- It records the usage frequencies of the symbols defined for every codebook, then calculates a mathematically optimal codeword assignment for them, minimizing the expected per-symbol bit cost, using the Huffman algorithm described in [this paper](https://dl.acm.org/doi/10.1145/3342555).
- It encapsulates Vorbis packets as tightly as possible into Ogg pages. Usual encoders extralimit themselves to support network live-streaming scenarios better, but these are not a concern on files.
- It removes any padding at the end of Vorbis packets and after Ogg pages. This can be useful because the reference encoder may pad audio packets with extra bytes when there is extreme pressure to meet a minimum bitrate and its bitrate management engine is used.
- It strips out non-Vorbis logical bitstreams, such as Ogg Skeleton metadata.
- Depending on the selected options, it may empty the vendor string and user comments in the Vorbis comment header.
- It drops to-be-discarded (i.e., zero-sized) audio packets.

# 📥 Installation

OptiVorbis is officially distributed in three ways:

- Within the project demo web page.
- As a command-line interface (CLI) application that both savvy end-users and other applications can use.
- As a Rust library that other Rust packages may use.

If you are an end-user looking to optimize files, either the demo web page or the CLI application is all you need. On the other hand, if you are a developer, you should look into the CLI or the Rust library.

## Demo web page

Just [visit the website](https://optivorbis.github.io/OptiVorbis): no installation required! Only a modern web browser that supports the required technologies is needed. Notably, Internet Explorer is not supported, but [you should no longer be using it](https://docs.microsoft.com/en-us/lifecycle/announcements/internet-explorer-11-end-of-support).

## CLI

Download the appropriate CLI executable from [GitHub Releases](https://github.com/OptiVorbis/OptiVorbis/releases), according to your operating system and CPU architecture. The executables are statically linked, so they are entirely self-contained and do not require any system configuration to run.

If you are using a Unix-like OS (Linux, macOS), remember that the executable must have the execute permission to work. This permission may be granted with the `chmod` command: `chmod +x optivorbis`.

If you want to go bleeding edge and try out code that was not yet released, you can get the latest CLI executables from the [GitHub Actions CI workflow](https://github.com/OptiVorbis/OptiVorbis/actions/workflows/ci.yml). No guarantees are made about the stability of these builds.

## Rust library

Like any other dependency, add it to the `Cargo.toml` manifest of your project, and start calling its API as you like. Check out its page at [`crates.io`](https://crates.io/crates/optivorbis) and the API docs at [`docs.rs`](https://docs.rs/optivorbis) to get started.

The minimum supported Rust version (MSRV) for every package in this repository is 1.61. Bumping this version is not considered a breaking change for semantic versioning purposes. We will try to do it only when we estimate that such a bump would not cause widespread inconvenience or breakage.

# 📕 Usage

By now, it should be obvious how to use the demo web page or Rust library. The CLI arguments follow a well-defined syntax. Several options are accepted to customize the optimization process and affect the output and operation of the CLI. The most important one to get started is `--help`, which shows the following usage help:

```
Usage:
    optivorbis [OPTION]... <input file> <output file or ->

Options:
    -h, --help          Prints information about the accepted command line
                        arguments and exits.
        --version       Prints version and copyright information, then exits.
    -q, --quiet         When enabled, the program will only print error
                        messages, unless -h is specified.
    -v, --verbose       Increases the verbosity of the messages. Can be
                        repeated several times.
    -r, --remuxer REMUXER
                        The remuxer to use for managing the encapsulation of
                        Vorbis streams in a container. If not specified, it
                        will be automatically deduced from the extension of
                        the output file.
                        Available remuxers: ogg2ogg
        --vendor_string_action VENDOR-STRING-ACTION
                        Changes how the vendor string contained in the Vorbis
                        identification header will be dealt with.
                        Available actions: copy, replace, appendTag,
                        appendShortTag, empty
        --comment_fields_action COMMENT-FIELDS-ACTION
                        Changes how the user comment fields contained in the
                        Vorbis comment header will be dealt with.
                        Available actions: copy, delete
        --remuxer_option OPTION=VALUE
                        Sets a remuxer-specific option to a value.
                        -----------------------
                        ogg2ogg remuxer options
                        -----------------------
                        - randomize_stream_serials=BOOLEAN
                        If set to true, the stream serials will be randomized,
                        following the intent of the Ogg specification. Set to
                        false to disable this behavior and have more control
                        over the serials. The default value is true.
                        - first_stream_serial_offset=INTEGER
                        A zero or positive integer that sets the offset that
                        will be added to the serial of the first stream. When
                        not randomizing stream serials, the offset matches the
                        serial that will be used for the first stream. The
                        default value is 0.
                        - ignore_start_sample_offset=BOOLEAN
                        Sets whether a non-zero calculated granule position
                        for the first audio sample will be honored when
                        recomputing granule positions in the generated Ogg
                        file or not. This usually is a good thing, but for
                        increased compatibility with some players or dedicated
                        purposes it may be advised to ignore this offset. The
                        default value is false.
                        - error_on_no_vorbis_streams=BOOLEAN
                        Sets whether not finding any Vorbis stream within the
                        Ogg container will be considered an error condition.
                        The default value is true, which means that not
                        finding any Vorbis stream will be considered an error.
                        This usually is the most desirable behavior.
```

# 📊 Testing and results

OptiVorbis has been developed as a part of a master's thesis and extensively tested before its public release with a dataset of 1783 randomly selected, valid Ogg Vorbis files downloaded from [Freesound](https://freesound.org/), totaling 164 hours and 16.58 GiB of sound data. OptiVorbis passed all the defined unit, integration, and system tests, achieving a size reduction for every analyzed file without altering the audio samples as decoded by `oggdec` (from the Debian `vorbis-tools` package, version 1.4.2). The only exceptions were four files that had their granule position data corruption fixed by OptiVorbis. The dataset comprised all kinds of Ogg Vorbis files containing mono, stereo, and surround sounds at varying sampling rates, generated by a wide variety of encoders. It was optimized in about half an hour, parallelizing the work across six CPU cores.

The following boxplot represents the distribution of relative per-file size reductions achieved by OptiVorbis.

<p align="center"><img src="https://i.imgur.com/teG5tPA.png" title="File size reductions boxplot" width="50%"/></p>

Even though the optimization potential may vary substantially, every file had its size reduced. The achieved reduction was significantly better than using generic compression tools on the Ogg Vorbis files, such as `xz`. Interestingly, the file size reduction distribution is slightly skewed towards higher reduction rates, as evidenced by the average (11.35%) being slightly higher than the median (9.62%). Half of the analyzed files had their size reduced between 4.91% and 13.44%.

Notably, the testing techniques included fuzzing, employing the AFL++ fuzzer. With the help of AFL++, the optimization of 600 million mutated Ogg Vorbis files was tested, discovering tens of failing runtime assertions that were fixed.

# ⚖️ License

The OptiVorbis library is licensed under either of

- GNU Affero General Public License, Version 3 (`LICENSE` or https://www.gnu.org/licenses/agpl-3.0.html)
- BSD 3-Clause "New" or "Revised" License (`LICENSE.BSD-3-Clause` or https://opensource.org/licenses/BSD-3-Clause)

at your option. The OptiVorbis CLI is licensed under the GNU Affero General Public License, Version 3 only. Other components (the web demo, etc.) are licensed according to the license files present in their root directory and their package metadata, if any.

# ✨ Contributing

Pull requests are accepted. Feel free to contribute if you can improve some aspect of the OptiVorbis project!

Contributions include, but are not limited to:

- Writing good bug reports or feature requests
- Sending a PR with code changes that implement an improvement or fix an issue
- Recommending OptiVorbis to others and engaging with the community
- Economically supporting the project (check out the "Sponsor" button in the GitHub page)

Code contributions must pass CI checks and be deemed of enough quality by a repository maintainer to be merged. OptiVorbis is structured as a standard Cargo workspace with several packages:

- `packages/optivorbis`: the OptiVorbis Rust library.
- `packages/optivorbis_afl_fuzz_target`: an AFL++ target for fuzzing OptiVorbis. See also the related `scripts/afl-fuzz.sh` script.
- `packages/optivorbis_cli`: the OptiVorbis CLI.
- `packages/vorbis_bitpack`: an implementation of bitwise reading and writing operations according to the Vorbis bitpacking convention.

The website (`web`) is a standard npm project that uses WebPack and the OptiVorbis library WASM target JS bindings, built with [`wasm-pack`](https://rustwasm.github.io/wasm-pack/).

# 🤝 Contact

We welcome friendly talk about the project, including questions, congratulations, and suggestions. Head to the [GitHub Discussions page](https://github.com/OptiVorbis/OptiVorbis/discussions) to interact with fellow users, contributors and developers.

# 🧑‍🤝‍🧑 Contributors

Thanks goes to these wonderful people ([emoji key](https://allcontributors.org/docs/en/emoji-key)):

<!-- ALL-CONTRIBUTORS-LIST:START - Do not remove or modify this section -->
<!-- prettier-ignore-start -->
<!-- markdownlint-disable -->
<table>
  <tr>
    <td align="center"><a href="https://github.com/AlexTMjugador"><img src="https://avatars.githubusercontent.com/u/7822554?v=4?s=100" width="100px;" alt=""/><br /><sub><b>Alejandro González</b></sub></a><br /><a href="https://github.com/OptiVorbis/OptiVorbis/commits?author=AlexTMjugador" title="Code">💻</a> <a href="https://github.com/OptiVorbis/OptiVorbis/commits?author=AlexTMjugador" title="Documentation">📖</a> <a href="#design-AlexTMjugador" title="Design">🎨</a> <a href="#ideas-AlexTMjugador" title="Ideas, Planning, & Feedback">🤔</a> <a href="#maintenance-AlexTMjugador" title="Maintenance">🚧</a> <a href="#projectManagement-AlexTMjugador" title="Project Management">📆</a> <a href="#data-AlexTMjugador" title="Data">🔣</a> <a href="#content-AlexTMjugador" title="Content">🖋</a> <a href="#research-AlexTMjugador" title="Research">🔬</a> <a href="#infra-AlexTMjugador" title="Infrastructure (Hosting, Build-Tools, etc)">🚇</a></td>
    <td align="center"><a href="https://github.com/victorlf4"><img src="https://avatars.githubusercontent.com/u/33629877?v=4?s=100" width="100px;" alt=""/><br /><sub><b>victorlf4</b></sub></a><br /><a href="#ideas-victorlf4" title="Ideas, Planning, & Feedback">🤔</a></td>
    <td align="center"><a href="https://github.com/MiguelDreamer"><img src="https://avatars.githubusercontent.com/u/31966940?v=4?s=100" width="100px;" alt=""/><br /><sub><b>Miguel</b></sub></a><br /><a href="#design-MiguelDreamer" title="Design">🎨</a></td>
    <td align="center"><a href="https://github.com/sya-ri"><img src="https://avatars.githubusercontent.com/u/34268371?v=4?s=100" width="100px;" alt=""/><br /><sub><b>sya-ri</b></sub></a><br /><a href="#ideas-sya-ri" title="Ideas, Planning, & Feedback">🤔</a> <a href="#infra-sya-ri" title="Infrastructure (Hosting, Build-Tools, etc)">🚇</a></td>
  </tr>
</table>

<!-- markdownlint-restore -->
<!-- prettier-ignore-end -->

<!-- ALL-CONTRIBUTORS-LIST:END -->

This project follows the [all-contributors](https://github.com/all-contributors/all-contributors) specification. Contributions of any kind welcome!

# ➡️ Related software

If you found OptiVorbis useful, you may be interested in programs such as:

- [`rogg`](https://gitlab.xiph.org/xiph/rogg): a small library for manipulating Ogg Vorbis files, which offers advanced and unusual features such as adjusting granule positions, replacing stream serial numbers, and recomputing the CRC of every Ogg page.
- [`oggz`](https://gitlab.xiph.org/xiph/liboggz): a library and set of command-line tools for low-level inspection and manipulation of Ogg Vorbis files.
- [`revorb`](https://github.com/ItsBranK/ReVorb): a command-line tool for recomputing granule positions of Ogg Vorbis files. OptiVorbis does such recomputation automatically, too.
- [`rehuff`](https://wiki.xiph.org/Rehuff): a proprietary, buggy, proof of concept program for optimizing Vorbis streams from 2002, written by Segher Boessenkool. OptiVorbis expands upon the optimization techniques said to be implemented in `rehuff`, offering a much more finished, reliable solution that is open-source.

# ⛔ Known limitations

As a concession to implementation simplicity, Vorbis streams that use the floor signal component format of type 0 are not supported. According to the Vorbis I specification, this format is "of limited modern use" and has been effectively deprecated by every known Vorbis encoder for more than 20 years, so streams with this floor type should be extremely rare to find. Pull requests that address this limitation are welcome.

Due to the two-pass optimization algorithm described above, OptiVorbis is not readily applicable for live-streaming use cases.

The Vorbis I setup header codebook format is vulnerable to denial of service attacks, as extremely dense prefix code trees, which take a significantly long time to parse, are valid according to the specification. OptiVorbis does not impose a depth or density limit in such trees, which guarantees its interoperability, but renders it vulnerable to specially-crafted files. This may be addressed in the future as information about the interoperability and mitigation impact of limiting the tree depth is gathered. In the meantime, applications dealing with untrusted files should be aware of this and resort to using OS features to bound resource consumption when applicable.
