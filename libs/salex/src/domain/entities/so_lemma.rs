use super::Lemma;
use super::{SentensEllerStilruta, SoLexem, Status, Uttal};

#[derive(Clone, Copy, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "lowercase")]
pub enum SoLemmaType {
    Lemma,
    Pekare,
    Variant,
    Vnomen,
    Bojform,
    Kortform,
    Abbrev,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SoBas {
    #[serde(flatten)]
    lemma: Lemma,
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
    pub relationer: Vec<SoLemmaRelation>,
    pub uttal: Vec<Uttal>,
}

impl SoBas {
    pub fn new(homograf_nr: Option<u32>, ortografi: String, visas: bool, status: Status) -> Self {
        let lemma = Lemma {
            homograf_nr,
            ortografi,
            visas,
            status,
        };
        Self { lemma }
    }
    pub fn with_ortografi_and_status(ortografi: String, status: Status) -> Self {
        Self::new(None, ortografi, true, status)
    }

    pub fn homograf_nr(&self) -> Option<u32> {
        self.lemma.homograf_nr
    }
    pub fn set_homograf_nr(&mut self, homograf_nr: Option<u32>) {
        self.lemma.homograf_nr = homograf_nr;
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SoLemmaRef {
    #[serde(flatten)]
    so_bas: SoBas,

}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct SoLemmaRelation {
    pub l_nr: u32,
    pub id_add: u32,
    pub typ: SoLemmaRelationsTyp,
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "snake_case", deny_unknown_fields)]
pub enum SoLemmaRelationsTyp {
    Moderverb,
    Aktivverb,
    Bojform,
    Pekare,
    Vnomen,
    RelateratVerb,
}
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SoLemma {
    #[serde(flatten)]
    so_bas: SoBas,
    #[serde(rename = "s_nr")]
    pub s_nr: u32,
    pub lemma_referenser: Vec<SoLemmaRef>,
    pub lexem: Vec<SoLexem>,
    pub sentenser_och_stilrutor: Vec<SentensEllerStilruta>,
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
        if self.so_bas.kommentar.is_empty() {
            self.so_bas.kommentar = kommentar.to_string();
        } else {
            self.so_bas.kommentar = format!("{};{}", self.so_bas.kommentar, kommentar);
        }
    }
}

pub struct SoLemmaBuilder {}

impl SoLemmaBuilder {
    pub fn new() -> Self {
        Self {}
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_create_so_lemma() {
        // SoLemma::new(
        //         1,
        //         s_nr: lemma_dto.s_nr,
        //         homograf_nr: lemma_dto.homograf_nr,
        //         ortografi: lemma_dto.ortografi,
        //         lemmatyp: lemma_dto.wtype,
        //         lemmaundertyp: lemma_dto.wsubtype,
        //         stam: lemma_dto.stem,
        //         böjning: lemma_dto.inflection,
        //         ordbildning: lemma_dto.ordbildning,
        //         analys: lemma_dto.analyzed.unwrap_or_else(|| String::new()),
        //         sorteringsform: lemma_dto.alfasort,
        //         kommentar: lemma_dto.kommentar.unwrap_or_else(|| String::new()),
        //         tagg: lemma_dto.tagg,
        //         ursprung,
        //         status,
        //         artikelkommentar,
        //         lemma_referenser: Vec::new(),
        //         lexem,
        //         relationer,
        //         sentenser_och_stilrutor,
        //         uttal,
        //         visas: lemma_dto.lm_sabob != 9,
        // )
    }

    #[test]
    fn can_create_so_lemma_from_builder() {
        // let _so_lemma = SoLemmaBuilder::new()
        //     .l_nr(1)
        //         .s_nr(2)
        //         .homograf_nr(lemma_dto.homograf_nr)
        //         .ortografi(lemma_dto.ortografi)
        //         .lemmatyp(lemma_dto.wtype)
        //         .lemmaundertyp(lemma_dto.wsubtype)
        //         .stam(lemma_dto.stem)
        //         .böjning(lemma_dto.inflection)
        //         .ordbildning(lemma_dto.ordbildning)
        //         .analys(lemma_dto.analyzed.unwrap_or_else(|| String::new()))
        //         .sorteringsform(lemma_dto.alfasort)
        //         .kommentar(lemma_dto.kommentar.unwrap_or_else(|| String::new()))
        //         .tagg(lemma_dto.tagg)
        //         ursprung,
        //         status,
        //         artikelkommentar,
        //         .lemma_referenser(Vec::new())
        //         lexem,
        //         relationer,
        //         sentenser_och_stilrutor,
        //         uttal,
        //         .visas(lemma_dto.lm_sabob != 9)
        //             .build();

    }
}
