use chrono::Utc;
use eyre::WrapErr;
use flate2::read::GzDecoder;
use flate2::write;
use salex::domain::entities::saol_lemma::LemmaTyp;
use salex::domain::entities::SoLemmaType;
use salex::domain::entities::Status;
use salex::{EntryDto, Superlemma};
use serde::Deserialize;
use serde_json::Value;
use std::collections::{HashMap, HashSet};
use std::io::{BufRead, Write};
use std::path::{Path, PathBuf};
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

#[derive(Clone, Debug, serde::Serialize)]
struct WtypeOrdklass {
    ortografi: String,
    wtype: SoLemmaType,
    ordklass: String,
    ursprung: String,
    kommentar: Option<String>,
}

pub fn lookup_wtype_ordklass(
    data_reader: &mut dyn io::BufRead,
    words_path: &Path,
    output_path: &Path,
) -> Result<(), Error> {
    log::trace!("lookup_wtype_ordklass called ...");
    let mut words = HashSet::new();
    let mut result: HashMap<String, Vec<WtypeOrdklass>> = HashMap::new();
    let mut csv_reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_path(words_path)?;

    let mut read_words = 0;
    for record in csv_reader.records() {
        let ortografi = record?;
        words.insert(ortografi.get(0).unwrap().to_owned());
        read_words += 1;
    }
    log::debug!("Words read: {}", read_words);

    let mut read_lines = 0;
    for line in data_reader.lines() {
        read_lines += 1;
        // log::trace!("line = {:?}", line);
        let entry: EntryDto<Superlemma> = serde_json::from_str(&line?)?;

        for so_lemma in &entry.entry.so_lemman {
            // log::debug!("so_lemma = {:?}", so_lemma);
            if so_lemma.status == Status::Förråd {
                continue;
            }
            if words.contains(&so_lemma.ortografi) {
                let ortografi = so_lemma.ortografi.clone();
                let data = WtypeOrdklass {
                    ortografi: so_lemma.ortografi.to_owned(),
                    wtype: so_lemma.lemmatyp,
                    ordklass: entry.entry.ordklass.to_owned(),
                    ursprung: "SO".into(),
                    kommentar: None,
                };
                log::debug!("data = {:?}", data);
                result
                    .entry(ortografi)
                    .and_modify(|e| e.push(data.clone()))
                    .or_insert(vec![data]);
            }
            for so_lemma_ref in &so_lemma.lemma_referenser {
                if words.contains(&so_lemma_ref.ortografi) {
                    let ortografi = so_lemma_ref.ortografi.clone();
                    let data = WtypeOrdklass {
                        ortografi: so_lemma_ref.ortografi.to_owned(),
                        wtype: so_lemma_ref.lemmatyp,
                        ordklass: entry.entry.ordklass.to_owned(),
                        ursprung: "SO".into(),
                        kommentar: None,
                    };
                    log::debug!("data = {:?}", data);
                    result
                        .entry(ortografi)
                        .and_modify(|e| e.push(data.clone()))
                        .or_insert(vec![data]);
                }
            }
        }

        for saol_lemma in &entry.entry.saol_lemman {
            if words.contains(&saol_lemma.ortografi) {
                let ortografi = saol_lemma.ortografi.clone();
                let wtype = match &saol_lemma.lemmatyp {
                    LemmaTyp::Lemma => SoLemmaType::Lemma,
                    LemmaTyp::Variant => SoLemmaType::Variant,
                    LemmaTyp::SeUnder => SoLemmaType::Pekare,
                    _ => todo!(),
                };
                let data = WtypeOrdklass {
                    ortografi: ortografi.clone(),
                    wtype,
                    ordklass: entry.entry.ordklass.to_owned(),
                    ursprung: "SAOL".into(),
                    kommentar: None,
                };
                log::debug!("data = {:?}", data);
                result
                    .entry(ortografi)
                    .and_modify(|e| e.push(data.clone()))
                    .or_insert(vec![data]);
            }

            for alt_form in &saol_lemma.alt {
                if words.contains(&alt_form.grundform) {
                    let ortografi = alt_form.grundform.clone();
                    let data = WtypeOrdklass {
                        ortografi: ortografi.clone(),
                        wtype: SoLemmaType::Variant,
                        ordklass: entry.entry.ordklass.to_owned(),
                        ursprung: "SAOL".into(),
                        kommentar: Some(format!("typ='{}'", alt_form.typ)),
                    };
                    log::debug!("data = {:?}", data);
                    result
                        .entry(ortografi)
                        .and_modify(|e| e.push(data.clone()))
                        .or_insert(vec![data]);
                }
            }
            for saol_lemma_ref in &saol_lemma.lemma_referenser {
                if words.contains(&saol_lemma_ref.ortografi) {
                    let ortografi = saol_lemma_ref.ortografi.clone();
                    let wtype = match &saol_lemma_ref.lemmatyp {
                        LemmaTyp::Lemma => SoLemmaType::Lemma,
                        LemmaTyp::Variant => SoLemmaType::Variant,
                        LemmaTyp::SeUnder => SoLemmaType::Pekare,
                        _ => todo!(),
                    };
                    let ordklass: String = match &saol_lemma_ref.ordklass {
                        Some(ref ordklass) => ordklass.clone(),
                        None => entry.entry.ordklass.clone(),
                    };
                    let data = WtypeOrdklass {
                        ortografi: ortografi.to_owned(),
                        wtype,
                        ordklass,
                        ursprung: "SAOL".into(),
                        kommentar: None,
                    };
                    log::debug!("data = {:?}", data);
                    result
                        .entry(ortografi)
                        .and_modify(|e| e.push(data.clone()))
                        .or_insert(vec![data]);
                }
            }
        }
    }
    log::info!("Lines read: {}", read_lines);

    let mut result_writer = csv::WriterBuilder::new()
        .has_headers(true)
        .from_path(output_path)?;
    let mut write_line_count = 0;
    for (_ortografi, findings) in &result {
        for finding in findings {
            result_writer.serialize(finding)?;
            write_line_count += 1;
        }
    }
    log::info!("Line written: {}", write_line_count);
    Ok(())
}

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UpdateValensDto {
    ortografi: String,
    ordklass: String,
    wtype: String,
    pre_valens: String,
    valens: String,
    vl_typ: String,
    fkom: String,
    def: String,
    deft: String,
    typ: String,
    hkom: String,
    vl_nr: u32,
    s_nr: u32,
    lm_sabob: u32,
    vl_status: u32,
    #[serde(rename = "Flytta till kc_nr")]
    flytta_till_kcnr: Option<u32>,
    #[serde(rename = "Kommentar")]
    kommentar: String,
}
pub fn update_valens(
    reader: &mut dyn io::BufRead,
    updates_path: &Path,
    output_path: &Path,
) -> eyre::Result<(), Error> {
    log::trace!("update_valens called ...");
    let mut read_line_count = 0;
    let mut read_updates = 0;
    let mut updates: HashMap<u32, UpdateValensDto> = HashMap::new();

    let mut csv_reader = csv::ReaderBuilder::new()
        .has_headers(true)
        .delimiter(b';')
        .from_path(updates_path)?;

    let mut read_words = 0;
    for record in csv_reader.deserialize() {
        // log::trace!("record = {:?}", record);
        let valens_dto: UpdateValensDto = record?;
        updates.insert(valens_dto.s_nr, valens_dto);
        read_updates += 1;
    }
    log::debug!("Updates read: {}", read_updates);

    let output = fs::File::create(output_path)
        .wrap_err_with(|| format!("Failed creating output '{}'", output_path.display()))?;
    let mut output = io::BufWriter::new(output);
    let mut write_line_count = 0;

    for line in reader.lines() {
        read_line_count += 1;
        let mut entry: EntryDto<Superlemma> = serde_json::from_str(&line?)?;
        let mut entry_updated = false;
        for so_lemma in &mut entry.entry.so_lemman {
            if let Some(update) = updates.get(&so_lemma.s_nr) {
                let mut kommentar = String::new();

                if let Some(kc_nr) = update.flytta_till_kcnr {
                    // log::debug!("update = {:#?}", update);
                    // log::debug!("so_lemma = {:#?}", so_lemma);
                    let mut valens = None;
                    for lexem in &mut so_lemma.lexem {
                        if let Some(pos) =
                            lexem.valenser.iter().position(|x| x.vl_nr == update.vl_nr)
                        {
                            valens = Some(lexem.valenser.remove(pos));
                            break;
                        }
                        log::debug!("update = {:#?}", update);
                        log::debug!("so_lemma = {:#?}", so_lemma);
                        todo!("look in cykler")
                    }
                    let mut valens = valens.expect("should have found valens");
                    valens.prevalens_def = update.pre_valens.clone();
                    if !update.kommentar.is_empty() {
                        if kommentar.is_empty() {
                            kommentar = update.kommentar.clone();
                        } else {
                            kommentar = format!("{};{}", kommentar, update.kommentar);
                        }
                    }
                    for lexem in &mut so_lemma.lexem {
                        if lexem.kc_nr == kc_nr {
                            lexem.valenser.push(valens.clone());
                            break;
                        }
                        let mut found_kc_nr = false;
                        for cykel in &mut lexem.cykler {
                            if cykel.kc_nr == kc_nr {
                                cykel.valenser.push(valens.clone());
                                found_kc_nr = true;
                                break;
                            }
                        }
                        if found_kc_nr {
                            break;
                        }
                    }
                    // log::debug!("so_lemma = {:#?}", so_lemma);
                    // continue;
                    entry_updated = true;
                } else {
                    for lexem in &mut so_lemma.lexem {
                        for valens in &mut lexem.valenser {
                            if valens.vl_nr == update.vl_nr {
                                // log::debug!("update = {:#?}", update);
                                // log::debug!("valens = {:#?}", valens);
                                valens.prevalens_def = update.pre_valens.clone();
                                // log::debug!("valens = {:#?}", valens);
                                if !update.kommentar.is_empty() {
                                    if kommentar.is_empty() {
                                        kommentar = update.kommentar.clone();
                                    } else {
                                        kommentar = format!("{};{}", kommentar, update.kommentar);
                                    }
                                }
                                entry_updated = true;
                            }
                        }
                        for cykel in &mut lexem.cykler {
                            for valens in &mut cykel.valenser {
                                if valens.vl_nr == update.vl_nr {
                                    // log::debug!("update = {:#?}", update);
                                    // log::debug!("valens = {:#?}", valens);
                                    valens.prevalens_def = update.pre_valens.clone();
                                    // log::debug!("valens = {:#?}", valens);
                                    if !update.kommentar.is_empty() {
                                        if kommentar.is_empty() {
                                            kommentar = update.kommentar.clone();
                                        } else {
                                            kommentar =
                                                format!("{};{}", kommentar, update.kommentar);
                                        }
                                    }
                                    entry_updated = true;
                                }
                            }
                        }
                    }
                }
                so_lemma.append_kommentar(kommentar.as_str());
            }
        }

        if entry_updated {
            entry.last_modified = Utc::now();
            entry.last_modified_by = "kristian.blensenius@gu.se".into();
            entry.message = "prevalens updated".into();
        }
        serde_json::to_writer(&mut output, &entry)?;
        writeln!(&mut output, "")?;
        write_line_count += 1;
    }
    log::info!("Line read: {}", read_line_count);

    log::info!("Line written: {}", write_line_count);
    Ok(())
}
