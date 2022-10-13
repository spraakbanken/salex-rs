#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Etymologi {
    #[serde(rename = "ety_nr")]
    ety_nr: u32,
    första_belägg: String,
    källa: String,
    beskrivning: String,
    kommentar: String,
    visas: bool,
}
