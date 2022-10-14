#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Valens {
    #[serde(rename = "vl_nr")]
    pub vl_nr: u32,
    pub typ: String,
    pub valens: String,
    pub prevalens_def: String,
    pub valens_def: String,
    pub kommentar: String,
    pub visas: bool,
}
