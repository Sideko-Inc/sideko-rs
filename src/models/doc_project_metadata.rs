#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct DocProjectMetadata {
    #[cfg_attr(feature = "cli", arg(id = "description", long = "description"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[cfg_attr(feature = "cli", arg(id = "title", long = "title"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}
