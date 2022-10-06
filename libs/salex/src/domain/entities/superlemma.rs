use super::{SaolLemma, SoLemma};
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct Superlemma {
    pub id: String,
    pub ortografi: String,
    pub ordklass: String,
    pub böjningsklass: String,
    pub kommentar: String,
    #[serde(rename = "SOLemman")]
    pub so_lemman: Vec<SoLemma>,
    #[serde(rename = "SAOLLemman")]
    pub saol_lemman: Vec<SaolLemma>,
}
