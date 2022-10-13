use super::{Etymologi, Hv, Idiom, Morfex, Syntex, Valens, Ämnesområde};
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SoLexem {
    #[serde(rename = "x_nr")]
    x_nr: u32,
    etymologier: Vec<Etymologi>,
    cykler: Vec<Cykel>,
    idiom: Vec<Idiom>,
    // py: Betydelse
    #[serde(rename = "kc_nr")]
    kc_nr: u32,
    definition: String,
    formkommentar: String,
    huvudkommentar: String,
    formkommentar_exempel: String,
    formkommentar_tillägg: String,
    slutkommentar: String,
    definitionstillägg: Option<String>,
    #[serde(rename = "ämnesområden")]
    amnesomraden: Vec<Ämnesområde>,
    hänvisningar: Vec<Hv>,
    morfex: Vec<Morfex>,
    syntex: Vec<Syntex>,
    valenser: Vec<Valens>,
    visas: bool,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Cykel {
    typ: String,
    // py: Betydelse
    #[serde(rename = "kc_nr")]
    kc_nr: u32,
    definition: String,
    formkommentar: String,
    huvudkommentar: String,
    formkommentar_exempel: String,
    formkommentar_tillägg: String,
    slutkommentar: String,
    definitionstillägg: Option<String>,
    #[serde(rename = "ämnesområden")]
    amnesomraden: Vec<Ämnesområde>,
    hänvisningar: Vec<Hv>,
    morfex: Vec<Morfex>,
    syntex: Vec<Syntex>,
    valenser: Vec<Valens>,
    visas: bool,
}
