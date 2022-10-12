#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SoLexem {
    #[serde(rename = "x_nr")]
    x_nr: u32,
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
pub struct Ämnesområde {
    #[serde(rename = "ämne")]
    amne: String,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Hv {
    typ: HänvisningsTyp,
    hänvisning: String,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub enum HänvisningsTyp {
    #[serde(rename = "SE:se")]
    Se,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Morfex {
    ortografi: String,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Syntex {
    text: String,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Valens {
    #[serde(rename = "vl_nr")]
    vl_nr: u32,
}
