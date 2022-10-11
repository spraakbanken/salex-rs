use async_trait::async_trait;

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct BFormDto {
    pub grundform: String,
    pub sortform: String,
    pub tagg: String,
    pub bklass: String,
    pub avstform: String,
    pub l_nr: String,
}

#[async_trait]
pub trait GetBForms {
    async fn query(&self, s_nr: i32, bklass: &str) -> Vec<BFormDto>;
}
