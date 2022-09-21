use env_logger::Env;
use std::path::PathBuf;
use std::{fs, io, process};
use std::io::BufRead;
use flate2::read::GzDecoder;
use serde_json::Value;
use std::collections::HashMap;

type Error = Box<dyn std::error::Error>;

#[derive(Clone, Debug, serde::Deserialize)]
pub struct KarpImportEntry {
    entry: Superlemma,
}

#[derive(Clone, Debug, serde::Deserialize)]
pub struct Superlemma {
    ortografi: String,
    ordklass: String,
    #[serde(rename="SOLemman")]
    so_lemman: Vec<serde_json::Value>,
    #[serde(rename="SAOLLemman")]
    saol_lemman: Vec<serde_json::Value>,
}

#[derive(serde::Serialize)]
struct Row<'a> {
    ortografi: &'a str,
    ordklass: &'a str,
    // Serde allows us to name our headers exactly,
    // even if they don't match our struct field names.
    num_saol_lemman: usize,
    num_so_lemman: usize,
}
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
            let input = submatches.get_one::<PathBuf>("input").expect("`input` is required");
            log::debug!("reading from '{}'", input.display());
            let output = submatches.get_one::<PathBuf>("output").expect("`output` is required");
            log::debug!("writing to '{}'", output.display());
            let fp_in = fs::File::open(input)?;
            let gz_in = GzDecoder::new(fp_in);
            let reader = io::BufReader::new(gz_in);
            let mut read_line_count = 0;
            let mut write_line_count = 0;
            let mut lemmas: HashMap<(String, String), (usize, usize)> = HashMap::new();
            let mut writer = csv::WriterBuilder::new()
                .has_headers(true)
                .from_path(output)?;
            for line in reader.lines() {
                read_line_count += 1;
                let entry: KarpImportEntry = serde_json::from_str(&line?)?;

                let (num_saol_lemman, num_so_lemman) = lemmas.entry((entry.entry.ortografi, entry.entry.ordklass))
                    .or_insert((0, 0));
                *num_saol_lemman += entry.entry.saol_lemman.len();
                *num_so_lemman += entry.entry.so_lemman.len();

            }
            log::info!("Line read: {}", read_line_count);
            for ((ortografi, ordklass), (num_saol_lemman, num_so_lemman)) in &lemmas {
                if num_saol_lemman == &0usize || num_so_lemman == &0usize {
                    continue;
                }
                if num_saol_lemman != num_so_lemman {
                    writer.serialize(Row {
                        ortografi,
                        ordklass,
                        num_saol_lemman: *num_saol_lemman,
                        num_so_lemman: *num_so_lemman,
                    })?;
                    write_line_count += 1;
                }
            }
            log::info!("Line written: {}", write_line_count);
        },
        _ => {
            unreachable!();
        }
    }
    Ok(())
}

fn cli() -> clap::Command<'static> {
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
                        .takes_value(true)
                        .help("jsonl.gz file to read")
                        .value_parser(clap::value_parser!(PathBuf))
                        .required(true)
                )
                .arg(
                    Arg::new("output")
                        .takes_value(true)
                        .help("csv file to write")
                        .value_parser(clap::value_parser!(PathBuf))
                        .required(true)
                )
        )
}
