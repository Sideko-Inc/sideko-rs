/// File Generated by Sideko (sideko.dev)
#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct ApiLinkDocVersion {
    #[cfg_attr(feature = "cli", arg(id = "doc-project-id", long = "doc-project-id"))]
    pub doc_project_id: String,
    #[cfg_attr(
        feature = "cli",
        arg(id = "doc-project-title", long = "doc-project-title")
    )]
    pub doc_project_title: String,
    #[cfg_attr(feature = "cli", arg(id = "id", long = "id"))]
    pub id: String,
    #[cfg_attr(feature = "cli", arg(id = "version", long = "version"))]
    pub version: i64,
}