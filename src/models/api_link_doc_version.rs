#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct ApiLinkDocVersion {
    #[cfg_attr(feature = "cli", arg(id = "doc-project-id", long = "doc-project-id"))]
    pub doc_project_id: String,
    #[cfg_attr(feature = "cli", arg(id = "doc-project-name", long = "doc-project-name"))]
    pub doc_project_name: String,
    #[cfg_attr(feature = "cli", arg(id = "id", long = "id"))]
    pub id: String,
    #[cfg_attr(feature = "cli", arg(id = "version", long = "version"))]
    pub version: i64,
}
