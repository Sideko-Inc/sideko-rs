#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum VersionOrBump {
    StrEnum(crate::models::VersionBumpEnum),
    Str(String),
}
impl Default for VersionOrBump {
    fn default() -> Self {
        VersionOrBump::Str(Default::default())
    }
}
