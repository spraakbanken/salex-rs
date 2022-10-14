use super::{Etymologi, Hv, Idiom, Morfex, Syntex, Valens, Ämnesområde};
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SoLexem {
    #[serde(rename = "x_nr")]
    pub x_nr: u32,
    pub etymologier: Vec<Etymologi>,
    pub cykler: Vec<Cykel>,
    pub idiom: Vec<Idiom>,
    // py: Betydelse
    #[serde(rename = "kc_nr")]
    pub kc_nr: u32,
    pub definition: String,
    pub formkommentar: String,
    pub huvudkommentar: String,
    pub formkommentar_exempel: String,
    pub formkommentar_tillägg: String,
    pub slutkommentar: String,
    pub definitionstillägg: Option<String>,
    #[serde(rename = "ämnesområden")]
    pub amnesomraden: Vec<Ämnesområde>,
    pub hänvisningar: Vec<Hv>,
    pub morfex: Vec<Morfex>,
    pub syntex: Vec<Syntex>,
    pub valenser: Vec<Valens>,
    pub visas: bool,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Cykel {
    pub typ: String,
    // py: Betydelse
    #[serde(rename = "kc_nr")]
    pub kc_nr: u32,
    pub definition: String,
    pub formkommentar: String,
    pub huvudkommentar: String,
    pub formkommentar_exempel: String,
    pub formkommentar_tillägg: String,
    pub slutkommentar: String,
    pub definitionstillägg: Option<String>,
    #[serde(rename = "ämnesområden")]
    pub amnesomraden: Vec<Ämnesområde>,
    pub hänvisningar: Vec<Hv>,
    pub morfex: Vec<Morfex>,
    pub syntex: Vec<Syntex>,
    pub valenser: Vec<Valens>,
    pub visas: bool,
}
