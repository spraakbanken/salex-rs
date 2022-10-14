#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Morfex {
    pub ortografi: String,
    pub hänvisning: String,
    pub kommentar: String,
    pub visas: bool,
}
