#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Syntex {
    pub typ: String,
    pub text: String,
    pub kommentar: String,
    pub visas: bool,
}
