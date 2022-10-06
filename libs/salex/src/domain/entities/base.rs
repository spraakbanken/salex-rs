#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum Status {
    Ny,
    Påbörjad,
    Granska,
    Ok,
    Förråd,
    #[serde(rename = "OKLAR STATUS")]
    OklarStatus,
}
