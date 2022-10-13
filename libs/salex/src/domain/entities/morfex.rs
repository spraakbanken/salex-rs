#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Morfex {
    ortografi: String,
    hänvisning: String,
    kommentar: String,
    visas: bool,
}
