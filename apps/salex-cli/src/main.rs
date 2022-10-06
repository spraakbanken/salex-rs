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
}
