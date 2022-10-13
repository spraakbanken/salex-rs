#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Syntex {
    typ: String,
    text: String,
    kommentar: String,
    visas: bool,
}
