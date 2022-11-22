#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SentensEllerStilruta {
    pub typ: String,
    pub text: String,
    pub origid: i32,
    pub orig_ord: String,
}
