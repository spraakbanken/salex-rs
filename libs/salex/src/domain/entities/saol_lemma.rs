use super::Status;

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SaolLemma {
    id: u32,
    ortografi: String,
    status: Status,
    visas: bool,
    homograf_nr: Option<u32>,
    analys: String,
    böjning: String,
    saol_klass: String,
    ptv: Option<bool>,
    smdb_n: Option<String>,
    hänvisningar: Vec<Hv>,
    lemma_referenser: Vec<SaolLemmaRef>,
    ursprung: String,
    fack: Vec<String>,
    alt: Vec<AltForm>,
    kommentar: Option<String>,
    lexem: Vec<SaolLexem>,
    relation: Vec<SaolRelation>,
    lemmatyp: LemmaTyp,
    fonetik: Vec<Fonetik>,
}

impl SaolLemma {
    pub fn new(
        id: u32,
        lemmatyp: LemmaTyp,
        ortografi: String,
        visas: bool,
        homograf_nr: Option<u32>,
        status: Status,
        hänvisningar: Vec<Hv>,
        ursprung: String,
        saol_klass: String,
        analys: String,
        böjning: String,
        ptv: Option<bool>,
        smdb_n: Option<String>,
        lemma_referenser: Vec<SaolLemmaRef>,
        fack: Vec<String>,
        alt: Vec<AltForm>,
        kommentar: Option<String>,
        lexem: Vec<SaolLexem>,
        relation: Vec<SaolRelation>,
        fonetik: Vec<Fonetik>,
    ) -> Self {
        Self {
            id,
            ortografi,
            saol_klass,
            analys,
            böjning,
            ptv,
            smdb_n,
            hänvisningar,
            lemma_referenser,
            ursprung,
            fack,
            alt,
            kommentar,
            lexem,
            relation,
            homograf_nr,
            status,
            visas,
            lemmatyp,
            fonetik,
        }
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn add_lemma_referens(&mut self, lemma_ref: &SaolLemmaRef) {
        self.lemma_referenser.push(lemma_ref.clone())
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct Hv {
    #[serde(rename = "type")]
    typ: String,
    hänvisning: String,
    visas: bool,
    text: Option<String>,
}

impl Hv {
    pub fn new(typ: String, hänvisning: String, visas: bool, text: Option<String>) -> Self {
        Self {
            typ,
            hänvisning,
            visas,
            text,
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SaolLemmaRef {
    id: u32,
    lemmatyp: LemmaTyp,
    ortografi: String,
    visas: bool,
    homograf_nr: Option<u32>,
    status: Status,
    hänvisningar: Vec<Hv>,
    ursprung: String,
    böjningsklass: Option<String>,
    ordklass: Option<String>,
    saol_klass: Option<String>,
    analys: Option<String>,
    böjning: Option<String>,
}

impl SaolLemmaRef {
    pub fn new(
        id: u32,
        lemmatyp: LemmaTyp,
        ortografi: String,
        visas: bool,
        homograf_nr: Option<u32>,
        status: Status,
        hänvisningar: Vec<Hv>,
        ursprung: String,
        saol_klass: Option<String>,
        analys: Option<String>,
        böjning: Option<String>,
        böjningsklass: Option<String>,
        ordklass: Option<String>,
    ) -> Self {
        Self {
            id,
            lemmatyp,
            visas,
            homograf_nr,
            status,
            hänvisningar,
            ortografi,
            ursprung,
            saol_klass,
            analys,
            böjning,
            böjningsklass,
            ordklass,
        }
    }

    pub fn ortografi(&self) -> &str {
        self.ortografi.as_str()
    }

    pub fn ordklass(&self) -> Option<&str> {
        self.ordklass.as_ref().map(|s| s.as_str())
    }

    pub fn böjningsklass(&self) -> Option<&str> {
        self.böjningsklass.as_ref().map(|s| s.as_str())
    }
}

impl From<SaolLemmaRef> for SaolLemma {
    fn from(l: SaolLemmaRef) -> Self {
        SaolLemma {
            id: l.id,
            lemmatyp: l.lemmatyp,
            ortografi: l.ortografi,
            visas: l.visas,
            status: Status::OklarStatus,
            homograf_nr: l.homograf_nr,
            hänvisningar: l.hänvisningar,
            saol_klass: l.saol_klass.unwrap_or_else(|| String::new()),
            analys: l.analys.unwrap_or_else(|| String::new()),
            böjning: l.böjning.unwrap_or_else(|| String::new()),
            ptv: None,
            smdb_n: None,
            lemma_referenser: Vec::new(),
            ursprung: l.ursprung,
            fack: Vec::new(),
            alt: Vec::new(),
            kommentar: None,
            lexem: Vec::new(),
            relation: Vec::new(),
            fonetik: Vec::new(),
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "lowercase")]
pub enum LemmaTyp {
    #[serde(rename = "se under")]
    SeUnder,
    Lemma,
    #[serde(alias = "variantform")]
    Variant,
    PartikelVerb,
}

impl From<&str> for LemmaTyp {
    fn from(s: &str) -> Self {
        match s {
            "se under" => Self::SeUnder,
            "variantform" => Self::Variant,
            "partikelverb" => Self::PartikelVerb,
            "lemma" => Self::Lemma,
            _ => panic!("Unknown LemmaTyp '{}'", s),
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AltForm {
    grundform: String,
    homograf_nr: Option<u32>,
    analys: Option<String>,
    #[serde(rename = "type")]
    typ: String,
    fonetik: Option<String>,
    böjning: Option<String>,
}

impl AltForm {
    pub fn new(
        grundform: String,
        homograf_nr: Option<u32>,
        analys: Option<String>,
        typ: String,
        fonetik: Option<String>,
        böjning: Option<String>,
    ) -> Self {
        Self {
            grundform,
            homograf_nr,
            analys,
            typ,
            fonetik,
            böjning,
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SaolLexem {
    id: String,
    definition: String,
    exempel: Vec<SaolExempel>,
    huvudkommentar: Option<String>,
    formkommentar: Vec<String>,
    hänvisningar: Vec<Hv>,
}

impl SaolLexem {
    pub fn new(
        id: String,
        definition: String,
        exempel: Vec<SaolExempel>,
        huvudkommentar: Option<String>,
        formkommentar: Vec<String>,
        hänvisningar: Vec<Hv>,
    ) -> Self {
        Self {
            id,
            definition,
            exempel,
            huvudkommentar,
            formkommentar,
            hänvisningar,
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SaolRelation {
    childtype: String,
    till_id: String,
}

impl SaolRelation {
    pub fn new(childtype: String, till_id: String) -> Self {
        Self { childtype, till_id }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct SaolExempel {
    text: String,
    parafras: Option<String>,
}

impl SaolExempel {
    pub fn new(text: String, parafras: Option<String>) -> Self {
        Self { text, parafras }
    }
}
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct Fonetik {
    form: String,
    kommentar: Option<String>,
    typ: Option<String>,
}

impl Fonetik {
    pub fn new(form: String, kommentar: Option<String>, typ: Option<String>) -> Self {
        Self {
            form,
            kommentar,
            typ,
        }
    }
}
