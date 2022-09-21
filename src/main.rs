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
    so_lemman: Vec<SoLemma>,
    #[serde(rename="SAOLLemman")]
    saol_lemman: Vec<SaolLemma>,
}

#[derive(Clone, Debug, serde::Deserialize)]
pub struct SoLemma {
    ortografi: String,
    lm_sabob: u32,
    visas: bool,
}

#[derive(Clone, Debug, serde::Deserialize)]
pub struct SaolLemma {
    ortografi: String,
    #[serde(rename="homografNr")]
    homograf_nr: Option<HomografNr>,
}

#[derive(Clone, Debug, serde::Deserialize)]
pub struct HomografNr {
    nummer: String,
    version: String
}

#[derive(serde::Serialize)]
struct OrtografiOrdklassRow<'a> {
    ortografi: &'a str,
    ordklass: &'a str,
    num_saol_lemman: usize,
    num_so_lemman: usize,
}

#[derive(serde::Serialize)]
struct OrtografiRow<'a> {
    ortografi: &'a str,
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
            let output_stub: &String = submatches.get_one("output_stub").expect("`outputstub` is required");
            let fp_in = fs::File::open(input)?;
            let gz_in = GzDecoder::new(fp_in);
            let reader = io::BufReader::new(gz_in);
            let mut read_line_count = 0;
            let mut lemmas_by_orto_ordk: HashMap<(String, String), (usize, usize)> = HashMap::new();
            let mut lemmas_by_orto: HashMap<String, (usize, usize)> = HashMap::new();

            for line in reader.lines() {
                read_line_count += 1;
                let entry: KarpImportEntry = serde_json::from_str(&line?)?;

                let sum_so_lemman = entry.entry.so_lemman
                    .iter()
                    .filter(|x| x.visas)
                    .fold(0, |acc, _| acc + 1);
                let (num_saol_lemman, num_so_lemman) = lemmas_by_orto_ordk.entry((entry.entry.ortografi.clone(), entry.entry.ordklass))
                    .or_insert((0, 0));
                *num_saol_lemman += entry.entry.saol_lemman.len();
                *num_so_lemman += sum_so_lemman;

                let (num_saol_lemman, num_so_lemman) = lemmas_by_orto.entry(entry.entry.ortografi)
                    .or_insert((0, 0));
                *num_saol_lemman += entry.entry.saol_lemman.len();
                *num_so_lemman += sum_so_lemman;

            }
            log::info!("Line read: {}", read_line_count);

            log::info!("Writing lemmas by ortografi and ordklass ...");
            let orto_ordk_path = format!("{}.ortografi.ordklass.csv", output_stub);
            log::debug!("writing to '{}'", orto_ordk_path);
            let mut orto_ordk_writer = csv::WriterBuilder::new()
                .has_headers(true)
                .from_path(orto_ordk_path)?;
            let mut write_line_count = 0;
            for ((ortografi, ordklass), (num_saol_lemman, num_so_lemman)) in &lemmas_by_orto_ordk {
                if num_saol_lemman == &0usize || num_so_lemman == &0usize {
                    continue;
                }
                if num_saol_lemman != num_so_lemman {
                    orto_ordk_writer.serialize(OrtografiOrdklassRow {
                        ortografi,
                        ordklass,
                        num_saol_lemman: *num_saol_lemman,
                        num_so_lemman: *num_so_lemman,
                    })?;
                    write_line_count += 1;
                }
            }
            log::info!("Line written: {}", write_line_count);

            log::info!("Writing lemmas by ortografi ...");
            let orto_path = format!("{}.ortografi.csv", output_stub);
            log::debug!("writing to '{}'", orto_path);

            let mut orto_writer = csv::WriterBuilder::new()
                .has_headers(true)
                .from_path(orto_path)?;
            let mut write_line_count = 0;
            for (ortografi, (num_saol_lemman, num_so_lemman)) in &lemmas_by_orto {
                if num_saol_lemman == &0usize || num_so_lemman == &0usize {
                    continue;
                }
                if num_saol_lemman != num_so_lemman {
                    orto_writer.serialize(OrtografiRow {
                        ortografi,
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
                    Arg::new("output_stub")
                        .takes_value(true)
                        .help("file_stub to write")
                        // .value_parser(clap::value_parser!(PathBuf))
                        .required(true)
                )
        )
}
