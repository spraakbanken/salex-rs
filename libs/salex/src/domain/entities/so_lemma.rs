use super::{SoLemmaRef, SoLemmaType, Status};

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SoLemma {
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
    #[serde(rename = "s_nr")]
    pub s_nr: u32,
    pub lemma_referenser: Vec<SoLemmaRef>,
}

// impl SoLemma {
//     pub fn new(l_nr: u32, ortografi: String, visas: bool) -> Self {
//         Self {
//             l_nr,
//             ortografi,
//             visas,
//         }
//     }
// }