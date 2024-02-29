#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Ämnesområde {
    #[serde(rename = "ämne")]
    pub amne: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub specifikt: Option<String>,
}
