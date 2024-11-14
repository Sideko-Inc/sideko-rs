#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct UpdateGuide {
    #[cfg_attr(feature = "cli", arg(id = "content", long = "content"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[cfg_attr(feature = "cli", arg(id = "nav-label", long = "nav-label"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nav_label: Option<String>,
    #[cfg_attr(feature = "cli", arg(id = "next-id", long = "next-id"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_id: Option<String>,
    #[cfg_attr(feature = "cli", arg(id = "prev-id", long = "prev-id"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prev_id: Option<String>,
    #[cfg_attr(feature = "cli", arg(id = "slug", long = "slug"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
}
