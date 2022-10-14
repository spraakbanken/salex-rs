#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Etymologi {
    #[serde(rename = "ety_nr")]
    pub ety_nr: u32,
    pub första_belägg: String,
    pub källa: String,
    pub beskrivning: String,
    pub kommentar: String,
    pub visas: bool,
}
