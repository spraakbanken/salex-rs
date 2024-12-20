use super::{
    SentensEllerStilruta, SoLemmaRef, SoLemmaRelation, SoLemmaType, SoLexem, Status, Uttal,
};

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SoLemma {
    // py: Lemma
    #[serde(skip_serializing_if = "Option::is_none")]
    pub homograf_nr: Option<u32>,
    pub ortografi: String,
    pub visas: bool,
    pub status: Status,
    // py: SoBas
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
    pub uttal: Vec<Uttal>,
    // py: SoLemma
    #[serde(rename = "s_nr")]
    pub s_nr: u32,
    pub lemma_referenser: Vec<SoLemmaRef>,
    pub lexem: Vec<SoLexem>,
    pub sentenser_och_stilrutor: Vec<SentensEllerStilruta>,
    pub relationer: Vec<SoLemmaRelation>,
    pub artikelkommentar: String,
}

impl SoLemma {
    //     pub fn new(l_nr: u32, ortografi: String, visas: bool) -> Self {
    //         Self {
    //             l_nr,
    //             ortografi,
    //             visas,
    //         }
    //     }
    pub fn append_kommentar(&mut self, kommentar: &str) {
        if self.kommentar.is_empty() {
            self.kommentar = kommentar.to_string();
        } else {
            self.kommentar = format!("{};{}", self.kommentar, kommentar);
        }
    }
}
