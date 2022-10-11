use super::so_base::SoLemmaRef;

pub struct SoOrd {
    pub ortografi: String,
    pub s_nr: i32,
    pub lemmaundertyp: String,
    pub sorteringsform: String,
    pub böjningsklass: String,
    pub lm_sabob: i32,
    pub lemma_referenser: Vec<SoLemmaRef>,
}
