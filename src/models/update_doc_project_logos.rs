#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct UpdateDocProjectLogos {
    #[cfg_attr(feature = "cli", arg(id = "dark", long = "dark"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dark: Option<String>,
    #[cfg_attr(feature = "cli", arg(id = "favicon", long = "favicon"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub favicon: Option<String>,
    #[cfg_attr(feature = "cli", arg(id = "light", long = "light"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub light: Option<String>,
}
