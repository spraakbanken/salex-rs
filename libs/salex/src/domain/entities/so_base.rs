use super::Status;

#[derive(Clone, Copy, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "lowercase")]
pub enum SoLemmaType {
    Lemma,
    Pekare,
    Variant,
    Vnomen,
    Bojform,
    Kortform,
    Abbrev,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SoLemmaRef {
    pub homograf_nr: Option<u32>,
    pub ortografi: String,
    pub visas: bool,
    pub status: Status,
    #[serde(rename = "l_nr")]
    pub l_nr: u32,
    pub analys: String,
    pub böjning: String,
    pub kommentar: String,
    pub lemmatyp: SoLemmaType,
    pub lemmaundertyp: String,
    pub ordbildning: String,
    pub sorteringsform: String,
    pub stam: String,
    pub tagg: String,
    pub ursprung: String,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SoLemmaRelation {
    #[serde(rename = "l_nr")]
    l_nr: u32,
    #[serde(rename = "i_add")]
    i_add: u32,
    typ: SoLemmaRelationsTyp,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "snake_case", deny_unknown_fields)]
pub enum SoLemmaRelationsTyp {
    Moderverb,
    Aktivverb,
    Bojform,
    Pekare,
    Vnomen,
    RelateratVerb,
}
