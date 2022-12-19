use env_logger::Env;
use flate2::read::GzDecoder;
use std::path::PathBuf;
use std::{fs, io, process};

type Error = Box<dyn std::error::Error>;

fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("trace"))
        .format_timestamp(None)
        .init();
    if let Err(err) = try_main() {
        log::error!("An error occured: {:?}", err);
        process::exit(1);
    }
}

fn try_main() -> Result<(), Error> {
    log::trace!("try_main");
    let matches = cli().get_matches();
    match matches.subcommand() {
        Some(("extract", submatches)) => {
            log::trace!("command 'extract'");
            let input = submatches
                .get_one::<PathBuf>("input")
                .expect("`input` is required");
            log::debug!("reading from '{}'", input.display());
            let output_stub: &String = submatches
                .get_one("output_stub")
                .expect("`outputstub` is required");
            let fp_in = fs::File::open(input)?;
            let gz_in = GzDecoder::new(fp_in);
            let mut reader = io::BufReader::new(gz_in);
            salex_cli::read_count_and_write(&mut reader, &output_stub)?;
        }
        Some(("lookup-wtype-ordklass", submatches)) => {
            log::trace!("command 'lookup'");
            let input = submatches
                .get_one::<PathBuf>("data")
                .expect("`data` is required");
            log::debug!("reading data from '{}'", input.display());
            let fp_in = fs::File::open(input)?;
            let gz_in = GzDecoder::new(fp_in);
            let mut data_reader = io::BufReader::new(gz_in);

            let words_path = submatches
                .get_one::<PathBuf>("words")
                .expect("`words` is required");
            log::debug!("reading words from '{}'", words_path.display());

            let output_path = submatches
                .get_one::<PathBuf>("output")
                .expect("`output` is required");

            log::debug!("writing results to '{}'", output_path.display());
            salex_cli::lookup_wtype_ordklass(&mut data_reader, &words_path, &output_path)?;
        }
        Some(("update-valens", submatches)) => {
            log::trace!("command 'lookup'");
            let input = submatches
                .get_one::<PathBuf>("data")
                .expect("`data` is required");
            log::debug!("reading data from '{}'", input.display());
            let fp_in = fs::File::open(input)?;
            let gz_in = GzDecoder::new(fp_in);
            let mut data_reader = io::BufReader::new(gz_in);

            let updates_path = submatches
                .get_one::<PathBuf>("updates")
                .expect("`updates` is required");
            log::debug!("reading updates from '{}'", updates_path.display());

            let output_path = submatches
                .get_one::<PathBuf>("output")
                .expect("`output` is required");

            log::debug!("writing results to '{}'", output_path.display());
            salex_cli::update_valens(&mut data_reader, &updates_path, &output_path)?;
        }
        _ => {
            unreachable!();
        }
    }
    Ok(())
}

fn cli() -> clap::Command {
    log::trace!("creating cli");
    use clap::{Arg, Command};
    Command::new(clap::crate_name!())
        .author(clap::crate_authors!("\n"))
        .version(clap::crate_version!())
        .about(clap::crate_description!())
        .arg_required_else_help(true)
        .subcommand(
            Command::new("extract")
                .about("extract data from jsonl.gz")
                .arg_required_else_help(true)
                .arg(
                    Arg::new("input")
                        .num_args(1)
                        .help("jsonl.gz file to read")
                        .value_parser(clap::value_parser!(PathBuf))
                        .required(true),
                )
                .arg(
                    Arg::new("output_stub")
                        .num_args(1)
                        .help("file_stub to write")
                        // .value_parser(clap::value_parser!(PathBuf))
                        .required(true),
                ),
        )
        .subcommand(
            Command::new("lookup-wtype-ordklass")
                .about("Lookup words in jsonl.gz")
                .arg_required_else_help(true)
                .arg(
                    Arg::new("data")
                        .num_args(1)
                        .help("jsonl.gz file to read")
                        .value_parser(clap::value_parser!(PathBuf))
                        .required(true),
                )
                .arg(
                    Arg::new("words")
                        .num_args(1)
                        .help("words to lookup")
                        .value_parser(clap::value_parser!(PathBuf))
                        .required(true),
                )
                .arg(
                    Arg::new("output")
                        .num_args(1)
                        .help("file to write to")
                        .value_parser(clap::value_parser!(PathBuf))
                        .required(true),
                ),
        )
        .subcommand(
            Command::new("update-valens")
                .about("Update valens in salex.jsonl.gz from csv")
                .arg_required_else_help(true)
                .arg(
                    Arg::new("input")
                        .num_args(1)
                        .help("jsonl.gz file to read")
                        .value_parser(clap::value_parser!(PathBuf))
                        .required(true),
                )
                .arg(
                    Arg::new("updates")
                        .num_args(1)
                        .help("updates to perform")
                        .value_parser(clap::value_parser!(PathBuf))
                        .required(true),
                )
                .arg(
                    Arg::new("output")
                        .num_args(1)
                        .help("file to write to")
                        .value_parser(clap::value_parser!(PathBuf))
                        .required(true),
                ),
        )
}
