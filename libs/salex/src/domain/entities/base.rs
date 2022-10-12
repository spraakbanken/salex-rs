#[derive(Clone, Copy, Debug, serde::Deserialize, serde::Serialize, PartialEq)]
#[serde(rename_all = "UPPERCASE")]
pub enum Status {
    Ny,
    #[serde(alias = "PÅBÖRJAD")]
    Påbörjad,
    Granska,
    Ok,
    #[serde(alias = "FÖRRÅD")]
    Förråd,
    #[serde(rename = "OKLAR STATUS")]
    OklarStatus,
}
