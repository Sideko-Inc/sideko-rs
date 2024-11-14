#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum IdOrInt {
    Str(String),
    Int(i64),
}
impl Default for IdOrInt {
    fn default() -> Self {
        IdOrInt::Int(Default::default())
    }
}
