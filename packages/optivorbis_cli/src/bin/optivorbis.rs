use std::borrow::Cow;
use std::env;
use std::fmt::Display;
use std::fs::File;
use std::io::{stdout, BufReader, BufWriter, Read, Seek, Write};
use std::path::Path;
use std::process::exit;
use std::str::FromStr;
use std::time::Instant;

use getopts::{Matches, Options, ParsingStyle};
use log::info;
use stderrlog::ColorChoice;

use optivorbis::remuxer::ogg_to_ogg;
use optivorbis::{
	OggToOgg, Remuxer, VorbisCommentFieldsAction, VorbisOptimizerSettings,
	VorbisVendorStringAction, OPTIVORBIS_VERSION_TAG
};

fn main() {
	exit(match run() {
		Ok(_) => 0,
		Err(err) => {
			eprintln!("{}", err);
			1
		}
	})
}

fn run() -> Result<(), Cow<'static, str>> {
	let mut options = Options::new();

	options
		.optflag(
			"h",
			"help",
			"Prints information about the accepted command line arguments and exits."
		)
		.optflag(
			"",
			"version",
			"Prints version and copyright information, then exits."
		)
		.optflag(
			"q",
			"quiet",
			"When enabled, the program will only print error messages, unless -h is specified."
		)
		.optflagmulti(
			"v",
			"verbose",
			"Increases the verbosity of the messages. Can be repeated several times."
		)
		.optopt(
			"r",
			"remuxer",
			"The remuxer to use for managing the encapsulation of Vorbis streams in a container. \
			If not specified, it will be automatically deduced from the extension of the output file.\n\
			Available remuxers: ogg2ogg",
			"REMUXER"
		)
		.optopt(
			"",
			"vendor_string_action",
			"Changes how the vendor string contained in the Vorbis identification header will be dealt with.\n\
			Available actions: copy, replace, appendTag, appendShortTag, empty",
			"VENDOR-STRING-ACTION"
		)
		.optopt(
			"",
			"comment_fields_action",
			"Changes how the user comment fields contained in the Vorbis comment header will be dealt with.\n\
			Available actions: copy, delete",
			"COMMENT-FIELDS-ACTION"
		)
		.optmulti(
			"",
			"remuxer_option",
			"Sets a remuxer-specific option to a value.\n\
			-----------------------\n\
			ogg2ogg remuxer options\n\
			-----------------------\n\
			- randomize_stream_serials=BOOLEAN\n\
			If set to true, the stream serials will be randomized, following the intent of the Ogg \
			specification. Set to false to disable this behavior and have more control over the serials. The \
			default value is true.\n\
			- first_stream_serial_offset=INTEGER\n\
			A zero or positive integer that sets the offset that will be added to the serial of the first \
			stream. When not randomizing stream serials, the offset matches the serial that will be used \
			for the first stream. The default value is 0.\n\
			- error_on_no_vorbis_streams=BOOLEAN\n\
			Sets whether not finding any Vorbis stream within the Ogg container will be considered an error \
			condition. The default value is true, which means that not finding any Vorbis stream will be \
			considered an error. This usually is the most desirable behavior.\n\
			- ignore_start_sample_offset=BOOLEAN\n\
			Sets whether a non-zero calculated granule position for the first audio sample will be honored \
			when recomputing granule positions in the generated Ogg file or not. This usually is a good \
			thing, but for increased compatibility with some players or dedicated purposes it may be advised \
			to ignore this offset. The default value is false.",
			"OPTION=VALUE"
		)
		.parsing_style(ParsingStyle::StopAtFirstFree);

	match options.parse(env::args().skip(1)) {
		Ok(matches) => {
			if matches.opt_present("h") {
				print_header();
				println!();
				println!("Usage:");
				print!(
					"    {} [OPTION]... <input file> <output file or ->",
					env!("CARGO_BIN_NAME")
				);
				println!("{}", options.usage(""));
			} else if matches.opt_present("version") {
				print_header();
			} else {
				let quiet_mode = matches.opt_present("q");

				if !quiet_mode {
					print_header();
					println!();
				}

				if matches.free.len() != 2 {
					return Err(format!(
						"Too many or few file arguments specified. Run {} -h to see command line argument help",
						env!("CARGO_BIN_NAME")
					))?;
				}

				let input_file_name = &*matches.free[0];
				let input_file = BufReader::new(
					File::open(input_file_name)
						.map_err(|err| format!("Could not open input file: {}", err))?
				);

				let output_file_name = &*matches.free[1];
				let (mut output_stdout, mut output_file);
				let (output_file, guessed_remuxer): (&mut dyn Write, Option<AvailableRemuxer>) =
					match output_file_name {
						"-" => {
							output_stdout = stdout().lock();

							(&mut output_stdout, None)
						}
						file_path => {
							output_file =
								BufWriter::new(File::create(file_path).map_err(|err| {
									format!("Could not open output file: {}", err)
								})?);

							let guessed_remuxer = match Path::new(file_path)
								.extension()
								.and_then(|extension| extension.to_str())
							{
								Some("ogg" | "oga" | "ogx") => Some(AvailableRemuxer::OggToOgg),
								_ => None
							};

							(&mut output_file, guessed_remuxer)
						}
					};

				let chosen_remuxer = matches
					.opt_get("remuxer")?
					.or(guessed_remuxer)
					.ok_or("No remuxer was specified, and no remuxer could be guessed from the file extension")?;

				init_logging(&matches, quiet_mode);

				remux(
					&matches,
					input_file,
					input_file_name,
					output_file,
					output_file_name,
					chosen_remuxer
				)?;
			}

			Ok(())
		}
		Err(parse_err) => Err(format!(
			"{}\nRun {} -h to see command line argument help",
			parse_err,
			env!("CARGO_BIN_NAME")
		))?
	}
}

fn remux<F: Read + Seek>(
	option_matches: &Matches,
	input_file: F,
	input_file_name: &str,
	output_file: &mut dyn Write,
	output_file_name: &str,
	chosen_remuxer: AvailableRemuxer
) -> Result<(), Cow<'static, str>> {
	macro_rules! set_remuxer_option_value {
		( $remuxer_settings:expr, $option:ident ) => {
			if let Some($option) = get_remuxer_option_value(option_matches, stringify!($option))? {
				$remuxer_settings.$option = $option;
			}
		};
	}

	let mut optimizer_settings = VorbisOptimizerSettings::default();

	macro_rules! set_optimizer_setting {
		( $field:ident, match { $( $string_value:expr => $value:expr ),+ } ) => {
			match option_matches.opt_str(stringify!($field)).as_deref() {
				$( Some($string_value) => optimizer_settings.$field = $value ),+,
				Some(value) => Err(format!("Invalid value for {} option: {}", stringify!($field), value))?,
				_ => ()
			}
		}
	}

	set_optimizer_setting!(comment_fields_action, match {
		"copy" => VorbisCommentFieldsAction::Copy,
		"delete" => VorbisCommentFieldsAction::Delete
	});
	set_optimizer_setting!(vendor_string_action, match {
		"copy" => VorbisVendorStringAction::Copy,
		"replace" => VorbisVendorStringAction::Replace,
		"appendTag" => VorbisVendorStringAction::AppendTag,
		"appendShortTag" => VorbisVendorStringAction::AppendShortTag,
		"empty" => VorbisVendorStringAction::Empty
	});

	match match chosen_remuxer {
		AvailableRemuxer::OggToOgg => {
			let mut remuxer_settings = ogg_to_ogg::Settings::default();
			set_remuxer_option_value!(remuxer_settings, randomize_stream_serials);
			set_remuxer_option_value!(remuxer_settings, first_stream_serial_offset);
			set_remuxer_option_value!(remuxer_settings, ignore_start_sample_offset);
			set_remuxer_option_value!(remuxer_settings, error_on_no_vorbis_streams);

			info!(
				"Processing {} and saving to {} with Ogg Vorbis remuxer...",
				input_file_name, output_file_name
			);

			let remux_begin = Instant::now();
			OggToOgg::new(remuxer_settings, optimizer_settings)
				.remux(input_file, output_file)
				.map(|_| remux_begin.elapsed())
		}
	} {
		Ok(duration) => {
			info!(
				"Optimization and repairs completed in {:.3} s. Have a nice day!",
				duration.as_secs_f64()
			);

			Ok(())
		}
		Err(err) => Err(format!("Error while optimizing the input file: {}", err))?
	}
}

fn get_remuxer_option_value<E: Display, T: FromStr<Err = E>>(
	option_matches: &Matches,
	option: &str
) -> Result<Option<T>, String> {
	option_matches
		.opt_strs("remuxer_option")
		.into_iter()
		.find_map(|option_and_value| {
			option_and_value
				.split_once('=')
				.and_then(|(parsed_option, value)| {
					if parsed_option == option {
						Some(value.parse())
					} else {
						None
					}
				})
		})
		.transpose()
		.map_err(|err| format!("Invalid value for {} remuxer option: {}", option, err))
}

fn init_logging(option_matches: &Matches, quiet_mode: bool) {
	let verbosity_level = option_matches.opt_count("v");

	stderrlog::new()
		.module("optivorbis")
		.verbosity(2 + verbosity_level)
		.show_level(false)
		.quiet(quiet_mode)
		.color(ColorChoice::Never)
		.init()
		.unwrap();
}

fn print_header() {
	println!("{}", OPTIVORBIS_VERSION_TAG);
	println!("{}", env!("CARGO_PKG_DESCRIPTION"));
	println!("Copyright (C) {}", env!("CARGO_PKG_AUTHORS"));
}

enum AvailableRemuxer {
	OggToOgg
}

impl FromStr for AvailableRemuxer {
	type Err = Cow<'static, str>;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"ogg2ogg" => Ok(Self::OggToOgg),
			_ => Err(format!("The specified remuxer is not valid: {}", s).into())
		}
	}
}
