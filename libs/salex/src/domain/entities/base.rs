#[derive(Clone, Copy, Debug, serde::Deserialize, serde::Serialize, PartialEq)]
#[serde(rename_all = "UPPERCASE")]
pub enum Status {
    Ny,
    #[serde(rename = "PÅBÖRJAD")]
    Påbörjad,
    Granska,
    Ok,
    #[serde(rename = "FÖRRÅD")]
    Förråd,
    #[serde(rename = "OKLAR STATUS")]
    OklarStatus,
}
