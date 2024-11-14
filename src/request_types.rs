#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct DeleteApiLinkRequest {
    #[cfg_attr(feature = "cli", arg(id = "id", long = "id"))]
    pub id: String,
}
#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct DeleteApiLinkGroupRequest {
    #[cfg_attr(feature = "cli", arg(id = "id", long = "id"))]
    pub id: String,
}
#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct DeleteDocProjectRequest {
    #[cfg_attr(feature = "cli", arg(id = "doc-name", long = "doc-name"))]
    pub doc_name: String,
}
#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct DeleteGuideRequest {
    #[cfg_attr(feature = "cli", arg(id = "doc-name", long = "doc-name"))]
    pub doc_name: String,
    #[cfg_attr(feature = "cli", arg(id = "doc-version", long = "doc-version"))]
    #[cfg_attr(
        feature = "cli",
        arg(value_parser = crate::core::clap::parse_json::<crate::models::IdOrInt>)
    )]
    pub doc_version: crate::models::IdOrInt,
    #[cfg_attr(feature = "cli", arg(id = "guide-id", long = "guide-id"))]
    pub guide_id: String,
}
#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct DeleteGuideHrefRequest {
    #[cfg_attr(feature = "cli", arg(id = "doc-name", long = "doc-name"))]
    pub doc_name: String,
    #[cfg_attr(feature = "cli", arg(id = "doc-version", long = "doc-version"))]
    #[cfg_attr(
        feature = "cli",
        arg(value_parser = crate::core::clap::parse_json::<crate::models::IdOrInt>)
    )]
    pub doc_version: crate::models::IdOrInt,
    #[cfg_attr(feature = "cli", arg(id = "guide-id", long = "guide-id"))]
    pub guide_id: String,
    #[cfg_attr(feature = "cli", arg(id = "variant", long = "variant"))]
    pub variant: crate::models::GuideHrefVariantEnum,
}
#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct DeleteAssetRequest {
    #[cfg_attr(feature = "cli", arg(id = "id", long = "id"))]
    pub id: String,
}
#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct DeleteServiceAccountRequest {
    #[cfg_attr(feature = "cli", arg(id = "id", long = "id"))]
    pub id: String,
}
#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct ListApiLinksRequest {
    #[cfg_attr(feature = "cli", arg(id = "doc-version", long = "doc-version"))]
    #[cfg_attr(
        feature = "cli",
        arg(value_parser = crate::core::clap::parse_json::<crate::models::IdOrInt>)
    )]
    pub doc_version: crate::models::IdOrInt,
}
#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct GetApiLinkRequest {
    #[cfg_attr(feature = "cli", arg(id = "id", long = "id"))]
    pub id: String,
}
#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct ListApiLinkGroupsRequest {
    #[cfg_attr(feature = "cli", arg(id = "doc-version", long = "doc-version"))]
    #[cfg_attr(
        feature = "cli",
        arg(value_parser = crate::core::clap::parse_json::<crate::models::IdOrInt>)
    )]
    pub doc_version: crate::models::IdOrInt,
}
#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct ExchangeCodeForKeyRequest {
    #[cfg_attr(feature = "cli", arg(id = "code", long = "code"))]
    pub code: String,
}
#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct LoginCallbackRequest {
    #[cfg_attr(feature = "cli", arg(id = "code", long = "code"))]
    pub code: String,
    #[cfg_attr(feature = "cli", arg(id = "state", long = "state"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}
#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct LoginUrlRequest {
    #[cfg_attr(feature = "cli", arg(id = "cli-output", long = "cli-output"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cli_output: Option<String>,
    #[cfg_attr(feature = "cli", arg(id = "cli-port", long = "cli-port"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cli_port: Option<i64>,
}
#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct CliCheckUpdatesRequest {
    #[cfg_attr(feature = "cli", arg(id = "cli-version", long = "cli-version"))]
    pub cli_version: String,
}
#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct GetDocProjectRequest {
    #[cfg_attr(feature = "cli", arg(id = "doc-name", long = "doc-name"))]
    pub doc_name: String,
}
#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct ListDeploymentsRequest {
    #[cfg_attr(feature = "cli", arg(id = "doc-name", long = "doc-name"))]
    pub doc_name: String,
    #[cfg_attr(feature = "cli", arg(id = "limit", long = "limit"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[cfg_attr(feature = "cli", arg(id = "target", long = "target"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<crate::models::DeploymentTargetEnum>,
}
#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct GetDeploymentRequest {
    #[cfg_attr(feature = "cli", arg(id = "doc-name", long = "doc-name"))]
    pub doc_name: String,
    #[cfg_attr(feature = "cli", arg(id = "deployment-id", long = "deployment-id"))]
    pub deployment_id: String,
}
#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct CheckPreviewRequest {
    #[cfg_attr(feature = "cli", arg(id = "doc-name", long = "doc-name"))]
    pub doc_name: String,
}
#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct GetDocProjectThemeRequest {
    #[cfg_attr(feature = "cli", arg(id = "doc-name", long = "doc-name"))]
    pub doc_name: String,
}
#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct ListDocVersionsRequest {
    #[cfg_attr(feature = "cli", arg(id = "doc-name", long = "doc-name"))]
    pub doc_name: String,
}
#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct GetDocVersionRequest {
    #[cfg_attr(feature = "cli", arg(id = "doc-name", long = "doc-name"))]
    pub doc_name: String,
    #[cfg_attr(feature = "cli", arg(id = "doc-version", long = "doc-version"))]
    #[cfg_attr(
        feature = "cli",
        arg(value_parser = crate::core::clap::parse_json::<crate::models::IdOrInt>)
    )]
    pub doc_version: crate::models::IdOrInt,
}
#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct ListGuidesRequest {
    #[cfg_attr(feature = "cli", arg(id = "doc-name", long = "doc-name"))]
    pub doc_name: String,
    #[cfg_attr(feature = "cli", arg(id = "doc-version", long = "doc-version"))]
    #[cfg_attr(
        feature = "cli",
        arg(value_parser = crate::core::clap::parse_json::<crate::models::IdOrInt>)
    )]
    pub doc_version: crate::models::IdOrInt,
}
#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct GetGuideRequest {
    #[cfg_attr(feature = "cli", arg(id = "doc-name", long = "doc-name"))]
    pub doc_name: String,
    #[cfg_attr(feature = "cli", arg(id = "doc-version", long = "doc-version"))]
    #[cfg_attr(
        feature = "cli",
        arg(value_parser = crate::core::clap::parse_json::<crate::models::IdOrInt>)
    )]
    pub doc_version: crate::models::IdOrInt,
    #[cfg_attr(feature = "cli", arg(id = "guide-id", long = "guide-id"))]
    pub guide_id: String,
}
#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct GetGuideContentRequest {
    #[cfg_attr(feature = "cli", arg(id = "doc-name", long = "doc-name"))]
    pub doc_name: String,
    #[cfg_attr(feature = "cli", arg(id = "doc-version", long = "doc-version"))]
    #[cfg_attr(
        feature = "cli",
        arg(value_parser = crate::core::clap::parse_json::<crate::models::IdOrInt>)
    )]
    pub doc_version: crate::models::IdOrInt,
    #[cfg_attr(feature = "cli", arg(id = "guide-id", long = "guide-id"))]
    pub guide_id: String,
}
#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct ListAssetsRequest {
    #[cfg_attr(feature = "cli", arg(id = "name", long = "name"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[cfg_attr(feature = "cli", arg(id = "page", long = "page"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
}
#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct GetServiceAccountRequest {
    #[cfg_attr(feature = "cli", arg(id = "id", long = "id"))]
    pub id: String,
}
#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct UpdateApiLinkRequest {
    #[cfg_attr(feature = "cli", arg(id = "id", long = "id"))]
    pub id: String,
    #[cfg_attr(feature = "cli", command(flatten))]
    pub data: crate::models::UpdateApiLink,
}
#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct UpdateApiLinkGroupRequest {
    #[cfg_attr(feature = "cli", arg(id = "id", long = "id"))]
    pub id: String,
    #[cfg_attr(feature = "cli", command(flatten))]
    pub data: crate::models::UpdateApiLinkGroup,
}
#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct UpdateDocProjectRequest {
    #[cfg_attr(feature = "cli", arg(id = "doc-name", long = "doc-name"))]
    pub doc_name: String,
    #[cfg_attr(feature = "cli", command(flatten))]
    pub data: crate::models::UpdateDocProject,
}
#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct UpdateGuideRequest {
    #[cfg_attr(feature = "cli", arg(id = "doc-name", long = "doc-name"))]
    pub doc_name: String,
    #[cfg_attr(feature = "cli", arg(id = "doc-version", long = "doc-version"))]
    #[cfg_attr(
        feature = "cli",
        arg(value_parser = crate::core::clap::parse_json::<crate::models::IdOrInt>)
    )]
    pub doc_version: crate::models::IdOrInt,
    #[cfg_attr(feature = "cli", arg(id = "guide-id", long = "guide-id"))]
    pub guide_id: String,
    #[cfg_attr(feature = "cli", command(flatten))]
    pub data: crate::models::UpdateGuide,
}
#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct UpdateAssetRequest {
    #[cfg_attr(feature = "cli", arg(id = "id", long = "id"))]
    pub id: String,
    #[cfg_attr(feature = "cli", command(flatten))]
    pub data: crate::models::UpdateAsset,
}
#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct CreateApiLinkRequest {
    #[cfg_attr(feature = "cli", command(flatten))]
    pub data: crate::models::NewApiLink,
}
#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct ReorderApiLinksRequest {
    #[cfg_attr(feature = "cli", command(flatten))]
    pub data: crate::models::ApiReorder,
}
#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct CreateApiLinkGroupRequest {
    #[cfg_attr(feature = "cli", command(flatten))]
    pub data: crate::models::NewApiLinkGroup,
}
#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct CreateDocProjectRequest {
    #[cfg_attr(feature = "cli", command(flatten))]
    pub data: crate::models::NewDocProject,
}
#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct TriggerDeploymentRequest {
    #[cfg_attr(feature = "cli", arg(id = "doc-name", long = "doc-name"))]
    pub doc_name: String,
    #[cfg_attr(feature = "cli", command(flatten))]
    pub data: crate::models::NewDeployment,
}
#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct CreateGuideRequest {
    #[cfg_attr(feature = "cli", arg(id = "doc-name", long = "doc-name"))]
    pub doc_name: String,
    #[cfg_attr(feature = "cli", arg(id = "doc-version", long = "doc-version"))]
    #[cfg_attr(
        feature = "cli",
        arg(value_parser = crate::core::clap::parse_json::<crate::models::IdOrInt>)
    )]
    pub doc_version: crate::models::IdOrInt,
    #[cfg_attr(feature = "cli", command(flatten))]
    pub data: crate::models::NewGuide,
}
#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct ReorderGuidesRequest {
    #[cfg_attr(feature = "cli", arg(id = "doc-name", long = "doc-name"))]
    pub doc_name: String,
    #[cfg_attr(feature = "cli", arg(id = "doc-version", long = "doc-version"))]
    #[cfg_attr(
        feature = "cli",
        arg(value_parser = crate::core::clap::parse_json::<crate::models::IdOrInt>)
    )]
    pub doc_version: crate::models::IdOrInt,
    #[cfg_attr(feature = "cli", arg(id = "data", long = "data"))]
    #[cfg_attr(
        feature = "cli",
        arg(value_parser = crate::core::clap::parse_json::<crate::models::ReorderGuide>)
    )]
    #[cfg_attr(feature = "cli", arg(num_args = 0.., value_delimiter = ' '))]
    pub data: Vec<crate::models::ReorderGuide>,
}
#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct CreateOrganizationRequest {
    #[cfg_attr(feature = "cli", command(flatten))]
    pub data: crate::models::NewOrganization,
}
#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct UploadAssetsRequest {
    #[cfg_attr(feature = "cli", command(flatten))]
    pub data: crate::models::File,
}
#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct InviteUserRequest {
    #[cfg_attr(feature = "cli", command(flatten))]
    pub data: crate::models::Invite,
}
#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct CreateServiceAccountRequest {
    #[cfg_attr(feature = "cli", command(flatten))]
    pub data: crate::models::NewServiceAccount,
}
#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct VercelWebhookRequest {
    #[cfg_attr(feature = "cli", arg(id = "data", long = "data"))]
    #[cfg_attr(
        feature = "cli",
        arg(value_parser = crate::core::clap::parse_json::<serde_json::Value>)
    )]
    pub data: serde_json::Value,
}
#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct UpdateDocProjectThemeRequest {
    #[cfg_attr(feature = "cli", arg(id = "doc-name", long = "doc-name"))]
    pub doc_name: String,
    #[cfg_attr(feature = "cli", command(flatten))]
    pub data: crate::models::ThemeValues,
}
#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct UpdateOrganizationThemeRequest {
    #[cfg_attr(feature = "cli", command(flatten))]
    pub data: crate::models::ThemeValues,
}
