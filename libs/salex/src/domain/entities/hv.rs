#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct Hv {
    typ: HänvisningsTyp,
    hänvisning: String,
    kommentar: String,
    visas: bool,
    l_nr: u32,
    kc_nr: u32,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub enum HänvisningsTyp {
    #[serde(rename = "SE:se")]
    Se,
    #[serde(rename = "JFR:jämför")]
    Jämför,
    #[serde(rename = "JFR:hyperonym")]
    Hyperonym,
    #[serde(rename = "JFR:hyponym")]
    Hyponym,
    #[serde(rename = "MOTSATS:antonym")]
    Antonym,
    #[serde(rename = "SYN:synonym")]
    Synonym,
    #[serde(rename = "JFR:cohyponym")]
    Cohyponym,
    #[serde(rename = "SE ÄVEN:av-/härledning")]
    Avledning,
}