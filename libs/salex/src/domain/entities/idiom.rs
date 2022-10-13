#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct Idiom {
    i_nr: u32,
    hänvisning: Option<u32>,
    idiom: String,
    formkommentar: String,
    alternativinledare: String,
    alternativform: String,
    idiombetydelser: Vec<IdiomBetydelse>,
    visas: bool,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct IdiomBetydelse {
    ix_nr: u32,
    definitionsinledare: String,
    huvudkommentar: String,
    definition: String,
    definitionstillägg: String,
    exempel: String,
    visas: bool,
}
