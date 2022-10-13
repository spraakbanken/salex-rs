#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Valens {
    #[serde(rename = "vl_nr")]
    vl_nr: u32,
    typ: String,
    valens: String,
    prevalens_def: String,
    valens_def: String,
    kommentar: String,
    visas: bool,
}
