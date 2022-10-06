#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct SoLemma {
    id: u32,
}

impl SoLemma {
    pub fn new(id: u32) -> Self {
        Self { id }
    }
}
