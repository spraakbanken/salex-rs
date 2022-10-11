#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct SoLemmaRef {
    pub l_nr: i32,
    pub lm_sabob: i32,
    pub ortografi: String,
    pub lemmatyp: LemmaRefTyp,
}


#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, PartialEq, PartialOrd, Ord, Eq)]
pub enum LemmaRefTyp {
    Lemma,
    Pekare,
    Variant,
    Vnomen,
    Bojform,
    Kortform,
    Abbrev,
}
