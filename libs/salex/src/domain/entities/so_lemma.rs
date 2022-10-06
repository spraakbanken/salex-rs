#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct SoLemma {
    id: u32,
    ortografi: String,
    pub visas: bool,
}

impl SoLemma {
    pub fn new(id: u32, ortografi: String, visas: bool) -> Self {
        Self {
            id,
            ortografi,
            visas,
        }
    }
}
