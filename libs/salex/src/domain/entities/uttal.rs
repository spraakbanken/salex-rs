#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Uttal {
    pub fonetikparentes: String,
    pub visas: bool,
    pub typ: String,
    pub lemma_med_tryckangivelse: String,
    pub fonetikkommentar: String,
    pub filnamn_inläst_uttal: String,
}
