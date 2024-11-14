#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct UpdateDocProjectSettingsActionButton {
    #[cfg_attr(feature = "cli", arg(id = "enabled", long = "enabled"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[cfg_attr(feature = "cli", arg(id = "label", long = "label"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[cfg_attr(feature = "cli", arg(id = "url", long = "url"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
