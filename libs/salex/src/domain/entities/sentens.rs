#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SentensEllerStilruta {
    typ: String,
    text: String,
    origid: u32,
    orig_ord: String,
}
