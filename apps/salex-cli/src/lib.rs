use flate2::read::GzDecoder;
use salex::{EntryDto, Superlemma};
use serde_json::Value;
use std::collections::HashMap;
use std::io::BufRead;
use std::path::PathBuf;
use std::{fs, io, process};

type Error = Box<dyn std::error::Error>;

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

pub fn read_count_and_write(reader: &mut dyn io::BufRead, output_stub: &str) -> Result<(), Error> {
    log::trace!("read_count_and_write called ...");
    let mut read_line_count = 0;
    let mut lemmas_by_orto_ordk: HashMap<(String, String), (usize, usize)> = HashMap::new();
    let mut lemmas_by_orto: HashMap<String, (usize, usize)> = HashMap::new();

    for line in reader.lines() {
        read_line_count += 1;
        let entry: EntryDto<Superlemma> = serde_json::from_str(&line?)?;

        let sum_so_lemman = entry
            .entry
            .so_lemman
            .iter()
            .filter(|x| x.visas)
            .fold(0, |acc, _| acc + 1);
        let (num_saol_lemman, num_so_lemman) = lemmas_by_orto_ordk
            .entry((entry.entry.ortografi.clone(), entry.entry.ordklass))
            .or_insert((0, 0));
        *num_saol_lemman += entry.entry.saol_lemman.len();
        *num_so_lemman += sum_so_lemman;

        let (num_saol_lemman, num_so_lemman) = lemmas_by_orto
            .entry(entry.entry.ortografi)
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
    Ok(())
}
