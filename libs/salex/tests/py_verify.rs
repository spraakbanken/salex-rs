use salex::Superlemma;
use std::{error::Error, fs, io};

#[test]
fn verify_data_generate_w_py() -> Result<(), Box<dyn Error>> {
    let file = fs::File::open("../../assets/data/lemman.json")?;
    let reader = io::BufReader::new(file);

    // let lemman: Vec<Superlemma> = serde_json::from_reader(reader)?;

    // assert_eq!(lemman.len(), 10);
    Ok(())
}
