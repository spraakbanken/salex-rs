#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Ämnesområde {
    #[serde(rename = "ämne")]
    amne: String,
}
