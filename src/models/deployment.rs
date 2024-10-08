/// File Generated by Sideko (sideko.dev)
#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct Deployment {
    #[cfg_attr(feature = "cli", arg(id = "created-at", long = "created-at"))]
    pub created_at: String,
    #[cfg_attr(feature = "cli", arg(id = "current-preview", long = "current-preview"))]
    pub current_preview: bool,
    #[cfg_attr(feature = "cli", arg(id = "current-prod", long = "current-prod"))]
    pub current_prod: bool,
    #[cfg_attr(feature = "cli", arg(id = "doc-version", long = "doc-version"))]
    #[cfg_attr(
        feature = "cli",
        arg(value_parser = crate::core::clap::parse_json::<crate::models::DocVersion>)
    )]
    pub doc_version: crate::models::DocVersion,
    #[cfg_attr(feature = "cli", arg(id = "id", long = "id"))]
    pub id: String,
    #[cfg_attr(feature = "cli", arg(id = "metadata", long = "metadata"))]
    #[cfg_attr(
        feature = "cli",
        arg(value_parser = crate::core::clap::parse_json::<serde_json::Value>)
    )]
    pub metadata: serde_json::Value,
    #[cfg_attr(feature = "cli", arg(id = "status", long = "status"))]
    pub status: crate::models::DeploymentStatusEnum,
    #[cfg_attr(feature = "cli", arg(id = "target", long = "target"))]
    pub target: crate::models::DeploymentTargetEnum,
}
