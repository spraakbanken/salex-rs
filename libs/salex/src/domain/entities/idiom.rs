#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct Idiom {
    pub i_nr: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hänvisning: Option<u32>,
    pub idiom: String,
    pub formkommentar: String,
    pub alternativinledare: String,
    pub alternativform: String,
    pub idiombetydelser: Vec<IdiomBetydelse>,
    pub visas: bool,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct IdiomBetydelse {
    pub ix_nr: u32,
    pub definitionsinledare: String,
    pub huvudkommentar: String,
    pub definition: String,
    pub definitionstillägg: String,
    pub exempel: String,
    pub visas: bool,
}
