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
    #[serde(default)]
    pub relationer: Vec<SoLemmaRelation>,
    pub uttal: Vec<Uttal>,
}

// impl Default for SoBas {
//     fn default() -> Self {
//         Self {
//             lemma: Lemma::default(),
//             l_nr: 0,
//             analys: String::default(),
//             böjning: String::default(),
//             kommentar: String::default(),
//             lemmatyp: SoLemmaType::default(),
//             lemmaundertyp: String::default(),
//             ordbildning: String::default(),
//             sorteringsform: String::default(),
//             stam: String::default(),
//             tagg: String::default(),
//             ursprung: String::default(),
//             relationer: Vec::default(),
//             uttal: Vec::default(),
//         }
//     }
// }

impl SoBas {
    // pub fn new(homograf_nr: Option<u32>, ortografi: String, visas: bool, status: Status) -> Self {
    //     let lemma = Lemma {
    //         homograf_nr,
    //         ortografi,
    //         visas,
    //         status,
    //     };
    //     Self { lemma }
    // }
    // pub fn with_ortografi_and_status(ortografi: String, status: Status) -> Self {
    //     Self::new(None, ortografi, true, status)
    // }
    pub fn default_lemma(l_nr: u32) -> Self {
        Self {
            lemma: Lemma::default(),
            l_nr: l_nr,
            analys: String::default(),
            böjning: String::default(),
            kommentar: String::default(),
            lemmatyp: SoLemmaType::Lemma,
            lemmaundertyp: String::default(),
            ordbildning: String::default(),
            sorteringsform: String::default(),
            stam: String::default(),
            tagg: String::default(),
            ursprung: String::default(),
            relationer: Vec::default(),
            uttal: Vec::default(),
        }
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
    pub fn with_lnr_and_snr(l_nr: u32, s_nr: u32) -> Self {
        let so_bas = SoBas::default_lemma(l_nr);
        Self {
            so_bas,
            s_nr,
            lemma_referenser: Vec::new(),
            lexem: Vec::new(),
            sentenser_och_stilrutor: Vec::new(),
            artikelkommentar: String::new(),
        }
    }
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
    pub fn l_nr(&self) -> u32 {
        self.so_bas.l_nr
    }
    pub fn s_nr(&self) -> u32 {
        self.s_nr
    }
}

pub struct SoLemmaBuilder {
    so_lemma: SoLemma,
}

impl SoLemmaBuilder {
    // pub fn new() -> Self {
    //     Self { so_lemma: SoLemma::empty() }
    // }
    pub fn with_lnr_and_snr(l_nr: u32, s_nr: u32) -> Self {
        Self {
            so_lemma: SoLemma::with_lnr_and_snr(l_nr, s_nr),
        }
    }

    pub fn l_nr(&mut self, l_nr: u32) -> &mut Self {
        self
    }
    pub fn build(self) -> SoLemma {
        self.so_lemma
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
        let so_lemma = SoLemmaBuilder::with_lnr_and_snr(1, 2)
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
            .build();
        assert_eq!(so_lemma.l_nr(), 1);
        assert_eq!(so_lemma.s_nr(), 1);
    }
}
