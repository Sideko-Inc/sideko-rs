#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct NewApiLinkGroup {
    #[cfg_attr(feature = "cli", arg(id = "doc-version-id", long = "doc-version-id"))]
    pub doc_version_id: String,
    #[cfg_attr(feature = "cli", arg(id = "nav-label", long = "nav-label"))]
    pub nav_label: String,
    #[cfg_attr(feature = "cli", arg(id = "slug", long = "slug"))]
    pub slug: String,
}
