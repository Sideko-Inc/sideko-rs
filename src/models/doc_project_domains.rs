#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct DocProjectDomains {
    #[cfg_attr(feature = "cli", arg(id = "preview", long = "preview"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preview: Option<String>,
    #[cfg_attr(feature = "cli", arg(id = "production", long = "production"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub production: Option<String>,
}
