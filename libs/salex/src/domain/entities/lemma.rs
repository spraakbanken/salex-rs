use super::Status;

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Lemma {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub homograf_nr: Option<u32>,
    pub status: Status,
    pub ortografi: String,
    pub visas: bool,
}
