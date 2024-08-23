pub struct BinaryResponse {
    pub content: bytes::Bytes,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct Error {
    pub error: ErrorErrorEnum,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct GetHealthResponse {
    pub ok: bool,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct GetPingResponse {
    pub ok: bool,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct ApiLinkApiVersion {
    pub api_project_id: String,
    pub api_project_title: String,
    pub id: String,
    pub semver: String,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct ApiLinkDocVersion {
    pub doc_project_id: String,
    pub doc_project_title: String,
    pub id: String,
    pub version: i64,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct ApiLinkGroup {
    pub doc_version_id: String,
    pub id: String,
    pub nav_label: String,
    pub order: i64,
    pub slug: String,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct ApiProject {
    pub created_at: String,
    pub id: String,
    pub title: String,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct ProjectMember {
    pub avatar_url: String,
    pub email: String,
    pub first_name: String,
    pub id: String,
    pub last_name: String,
    pub role: ProjectRoleEnum,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct ApiMockServer {
    pub enabled: bool,
    pub url: String,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct Validation {
    pub message: String,
    pub severity: ValidationSeverityEnum,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct Stats {
    pub authenticated_methods: f64,
    pub authentication_schemes: Vec<String>,
    pub endpoints: f64,
    pub methods: f64,
    pub public_methods: f64,
    pub response_codes: Vec<f64>,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct UserApiKey {
    pub api_key: String,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct CliUpdate {
    pub message: String,
    pub severity: CliUpdateSeverityEnum,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct DocProjectDomains {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preview: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub production: Option<String>,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct Asset {
    pub extension: String,
    pub id: String,
    pub name: String,
    pub url: String,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct DocProjectActionButton {
    pub enabled: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct DocProjectMetadata {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct DocVersion {
    pub created_at: String,
    pub doc_project_id: String,
    pub id: String,
    pub status: DocVersionStatusEnum,
    pub version: i64,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct ThemeValues {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_reference_group_variant: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dark_active_button_bg_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dark_active_button_text_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dark_bg_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dark_navbar_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dark_navbar_text_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub light_active_button_bg_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub light_active_button_text_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub light_bg_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub light_navbar_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub light_navbar_text_color: Option<String>,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct GuideWithChildren {
    pub children: Vec<Box<GuideWithChildren>>,
    pub created_at: String,
    pub id: String,
    pub is_parent: bool,
    pub nav_label: String,
    pub order: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    pub slug: String,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct GuideHref {
    pub id: String,
    pub nav_label: String,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct GuideContent {
    pub content: String,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct OrganizationFeatures {
    pub max_api_projects: i64,
    pub max_doc_projects: i64,
    pub max_mock_servers: i64,
    pub max_teamates: i64,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct Pagination {
    pub page: i64,
    pub page_count: i64,
    pub page_limit: i64,
    pub total_count: i64,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct OrganizationMember {
    pub avatar_url: String,
    pub email: String,
    pub first_name: String,
    pub id: String,
    pub last_name: String,
    pub role: OrganizationRoleEnum,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct SdkProject {
    pub api_project_version_id: String,
    pub language: GenerationLanguageEnum,
    pub name: String,
    pub semver: String,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct User {
    pub avatar_url: String,
    pub email: String,
    pub first_name: String,
    pub id: String,
    pub last_name: String,
    pub organization_role: OrganizationRoleEnum,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct UserProjectRole {
    pub project_id_or_name: String,
    pub project_type: ProjectTypeEnum,
    pub role: ProjectRoleEnum,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct ServiceAccount {
    pub api_key: String,
    pub created_at: String,
    pub id: String,
    pub name: String,
    pub project_roles: Vec<UserProjectRole>,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct UpdateApiLink {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_version_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_request_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_mock_server: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nav_label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<ApiLinkPolicyEnum>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct UpdateApiLinkGroup {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nav_label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct UpdateApiProject {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct UpdateApiVersion {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mock_server_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub openapi: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub semver: Option<String>,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct UpdateDocProjectLogos {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dark: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub favicon: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub light: Option<String>,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct UpdateDocProjectSettingsActionButton {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct UpdateDocProjectSettingsMetadata {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct UpdateGuide {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nav_label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prev_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct UpdateAsset {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct LatestApiLinkPolicy {
    pub api_project_id: String,
    #[serde(rename = "type")]
    pub type_field: LatestApiLinkPolicyTypeEnum,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct PinnedApiLinkPolicy {
    pub api_version_id: String,
    #[serde(rename = "type")]
    pub type_field: PinnedApiLinkPolicyTypeEnum,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct ApiLinkGroupReorder {
    pub id: String,
    pub order: i64,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct ApiLinkReorder {
    pub group_id: String,
    pub id: String,
    pub order: i64,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct NewApiLinkGroup {
    pub doc_version_id: String,
    pub nav_label: String,
    pub slug: String,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct NewApiProject {
    pub title: String,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct NewProjectRole {
    pub role: ProjectRoleEnum,
    pub user_id: String,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct NewApiVersion {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mock_server_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    pub openapi: String,
    pub semver: String,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct NewDocProject {
    pub title: String,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct NewDeployment {
    pub doc_version_id: String,
    pub target: DeploymentTargetEnum,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct NewGuide {
    pub content: String,
    pub is_parent: bool,
    pub nav_label: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prev_id: Option<String>,
    pub slug: String,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct ReorderGuide {
    pub id: String,
    pub order: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct NewOrganization {
    pub name: String,
    pub subdomain: String,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct File {
    pub file: String,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct NewSdkResponse {
    pub patch: String,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct StatelessGenerateSdk {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_url: Option<String>,
    pub language: GenerationLanguageEnum,
    pub openapi: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tests_mock_server_url: Option<String>,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct Invite {
    pub email: String,
    pub role: OrganizationRoleEnum,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct CreateServiceAccount {
    pub name: String,
    pub project_roles: Vec<UserProjectRole>,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct ApiLink {
    pub api_version: ApiLinkApiVersion,
    pub build_request_enabled: bool,
    pub created_at: String,
    pub doc_version: ApiLinkDocVersion,
    pub group_id: String,
    pub id: String,
    pub include_mock_server: bool,
    pub nav_label: String,
    pub order: i64,
    pub policy: ApiLinkPolicyEnum,
    pub slug: String,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct ApiVersion {
    pub api_project_id: String,
    pub created_at: String,
    pub id: String,
    pub mock_server: ApiMockServer,
    pub notes: String,
    pub semver: String,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct OpenApi {
    pub is_config_valid: bool,
    pub is_valid: bool,
    pub openapi: String,
    pub validations: Vec<Validation>,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct DocProjectLogos {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dark: Option<Asset>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub favicon: Option<Asset>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub light: Option<Asset>,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct DocProjectSettings {
    pub action_button: DocProjectActionButton,
    pub metadata: DocProjectMetadata,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct Deployment {
    pub created_at: String,
    pub current_preview: bool,
    pub current_prod: bool,
    pub doc_version: DocVersion,
    pub id: String,
    pub metadata: serde_json::Value,
    pub status: DeploymentStatusEnum,
    pub target: DeploymentTargetEnum,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct Theme {
    pub owner: ThemeOwnerEnum,
    pub values: ThemeValues,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct Guide {
    pub created_at: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_next_href: Option<GuideHref>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_prev_href: Option<GuideHref>,
    pub id: String,
    pub is_parent: bool,
    pub nav_label: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_href: Option<GuideHref>,
    pub order: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prev_href: Option<GuideHref>,
    pub slug: String,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct Organization {
    pub features: OrganizationFeatures,
    pub id: String,
    pub name: String,
    pub subdomain: String,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct ListAssetsPage {
    pub pagination: Pagination,
    pub results: Vec<Asset>,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct UpdateDocProjectSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_button: Option<UpdateDocProjectSettingsActionButton>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<UpdateDocProjectSettingsMetadata>,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct NewApiLink {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_version_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_request_enabled: Option<bool>,
    pub doc_version_id: String,
    pub group_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_mock_server: Option<bool>,
    pub nav_label: String,
    pub policy: Union,
    pub slug: String,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct ApiReorder {
    pub doc_version_id: String,
    pub groups: Vec<ApiLinkGroupReorder>,
    pub links: Vec<ApiLinkReorder>,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct OrganizationWithRedirect {
    pub organization: Organization,
    pub redirect_to: String,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct DocProject {
    pub created_at: String,
    pub current_version_id: String,
    pub domains: DocProjectDomains,
    pub id: String,
    pub logos: DocProjectLogos,
    pub settings: DocProjectSettings,
    pub title: String,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct UpdateDocProject {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logos: Option<UpdateDocProjectLogos>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<UpdateDocProjectSettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum Union {
    LatestApiLinkPolicy(LatestApiLinkPolicy),
    PinnedApiLinkPolicy(PinnedApiLinkPolicy),
}
impl Default for Union {
    fn default() -> Self {
        Union::PinnedApiLinkPolicy(Default::default())
    }
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub enum ErrorErrorEnum {
    #[default]
    #[serde(rename = "Bad Request")]
    BadRequest,
    #[serde(rename = "forbidden")]
    Forbidden,
    #[serde(rename = "insufficient_features")]
    InsufficientFeatures,
    #[serde(rename = "internal_server_error")]
    InternalServerError,
    #[serde(rename = "invalid_openapi")]
    InvalidOpenapi,
    #[serde(rename = "invalid_url")]
    InvalidUrl,
    #[serde(rename = "not_found")]
    NotFound,
    #[serde(rename = "unauthorized")]
    Unauthorized,
    #[serde(rename = "unavailable_subdomain")]
    UnavailableSubdomain,
}
impl std::fmt::Display for ErrorErrorEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            ErrorErrorEnum::BadRequest => "Bad Request",
            ErrorErrorEnum::Forbidden => "forbidden",
            ErrorErrorEnum::InsufficientFeatures => "insufficient_features",
            ErrorErrorEnum::InternalServerError => "internal_server_error",
            ErrorErrorEnum::InvalidOpenapi => "invalid_openapi",
            ErrorErrorEnum::InvalidUrl => "invalid_url",
            ErrorErrorEnum::NotFound => "not_found",
            ErrorErrorEnum::Unauthorized => "unauthorized",
            ErrorErrorEnum::UnavailableSubdomain => "unavailable_subdomain",
        };
        write!(f, "{}", str_val)
    }
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub enum GuideHrefVariantEnum {
    #[default]
    #[serde(rename = "next")]
    Next,
    #[serde(rename = "prev")]
    Prev,
}
impl std::fmt::Display for GuideHrefVariantEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            GuideHrefVariantEnum::Next => "next",
            GuideHrefVariantEnum::Prev => "prev",
        };
        write!(f, "{}", str_val)
    }
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub enum ApiLinkPolicyEnum {
    #[default]
    #[serde(rename = "latest")]
    Latest,
    #[serde(rename = "pinned")]
    Pinned,
}
impl std::fmt::Display for ApiLinkPolicyEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            ApiLinkPolicyEnum::Latest => "latest",
            ApiLinkPolicyEnum::Pinned => "pinned",
        };
        write!(f, "{}", str_val)
    }
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub enum ProjectRoleEnum {
    #[default]
    #[serde(rename = "admin")]
    Admin,
    #[serde(rename = "contributor")]
    Contributor,
    #[serde(rename = "viewer")]
    Viewer,
}
impl std::fmt::Display for ProjectRoleEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            ProjectRoleEnum::Admin => "admin",
            ProjectRoleEnum::Contributor => "contributor",
            ProjectRoleEnum::Viewer => "viewer",
        };
        write!(f, "{}", str_val)
    }
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub enum ValidationSeverityEnum {
    #[default]
    #[serde(rename = "error")]
    Error,
    #[serde(rename = "info")]
    Info,
    #[serde(rename = "warning")]
    Warning,
}
impl std::fmt::Display for ValidationSeverityEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            ValidationSeverityEnum::Error => "error",
            ValidationSeverityEnum::Info => "info",
            ValidationSeverityEnum::Warning => "warning",
        };
        write!(f, "{}", str_val)
    }
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub enum CliUpdateSeverityEnum {
    #[default]
    #[serde(rename = "info")]
    Info,
    #[serde(rename = "required")]
    Required,
    #[serde(rename = "suggested")]
    Suggested,
}
impl std::fmt::Display for CliUpdateSeverityEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            CliUpdateSeverityEnum::Info => "info",
            CliUpdateSeverityEnum::Required => "required",
            CliUpdateSeverityEnum::Suggested => "suggested",
        };
        write!(f, "{}", str_val)
    }
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub enum DeploymentTargetEnum {
    #[default]
    #[serde(rename = "Preview")]
    Preview,
    #[serde(rename = "Production")]
    Production,
}
impl std::fmt::Display for DeploymentTargetEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            DeploymentTargetEnum::Preview => "Preview",
            DeploymentTargetEnum::Production => "Production",
        };
        write!(f, "{}", str_val)
    }
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub enum DocVersionStatusEnum {
    #[default]
    #[serde(rename = "Draft")]
    Draft,
    #[serde(rename = "Published")]
    Published,
    #[serde(rename = "Publishing")]
    Publishing,
}
impl std::fmt::Display for DocVersionStatusEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            DocVersionStatusEnum::Draft => "Draft",
            DocVersionStatusEnum::Published => "Published",
            DocVersionStatusEnum::Publishing => "Publishing",
        };
        write!(f, "{}", str_val)
    }
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub enum DeploymentStatusEnum {
    #[default]
    #[serde(rename = "Building")]
    Building,
    #[serde(rename = "Cancelled")]
    Cancelled,
    #[serde(rename = "Complete")]
    Complete,
    #[serde(rename = "Created")]
    Created,
    #[serde(rename = "Error")]
    Error,
    #[serde(rename = "Generated")]
    Generated,
}
impl std::fmt::Display for DeploymentStatusEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            DeploymentStatusEnum::Building => "Building",
            DeploymentStatusEnum::Cancelled => "Cancelled",
            DeploymentStatusEnum::Complete => "Complete",
            DeploymentStatusEnum::Created => "Created",
            DeploymentStatusEnum::Error => "Error",
            DeploymentStatusEnum::Generated => "Generated",
        };
        write!(f, "{}", str_val)
    }
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub enum ThemeOwnerEnum {
    #[default]
    #[serde(rename = "default")]
    Default,
    #[serde(rename = "organization")]
    Organization,
    #[serde(rename = "self")]
    _Self,
}
impl std::fmt::Display for ThemeOwnerEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            ThemeOwnerEnum::Default => "default",
            ThemeOwnerEnum::Organization => "organization",
            ThemeOwnerEnum::_Self => "self",
        };
        write!(f, "{}", str_val)
    }
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub enum OrganizationRoleEnum {
    #[default]
    #[serde(rename = "admin")]
    Admin,
    #[serde(rename = "manager")]
    Manager,
    #[serde(rename = "member")]
    Member,
}
impl std::fmt::Display for OrganizationRoleEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            OrganizationRoleEnum::Admin => "admin",
            OrganizationRoleEnum::Manager => "manager",
            OrganizationRoleEnum::Member => "member",
        };
        write!(f, "{}", str_val)
    }
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub enum GenerationLanguageEnum {
    #[default]
    #[serde(rename = "go")]
    Go,
    #[serde(rename = "python")]
    Python,
    #[serde(rename = "ruby")]
    Ruby,
    #[serde(rename = "rust")]
    Rust,
    #[serde(rename = "typescript")]
    Typescript,
}
impl std::fmt::Display for GenerationLanguageEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            GenerationLanguageEnum::Go => "go",
            GenerationLanguageEnum::Python => "python",
            GenerationLanguageEnum::Ruby => "ruby",
            GenerationLanguageEnum::Rust => "rust",
            GenerationLanguageEnum::Typescript => "typescript",
        };
        write!(f, "{}", str_val)
    }
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub enum ProjectTypeEnum {
    #[default]
    #[serde(rename = "api")]
    Api,
    #[serde(rename = "documentation")]
    Documentation,
}
impl std::fmt::Display for ProjectTypeEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            ProjectTypeEnum::Api => "api",
            ProjectTypeEnum::Documentation => "documentation",
        };
        write!(f, "{}", str_val)
    }
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub enum LatestApiLinkPolicyTypeEnum {
    #[default]
    #[serde(rename = "latest")]
    Latest,
}
impl std::fmt::Display for LatestApiLinkPolicyTypeEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            LatestApiLinkPolicyTypeEnum::Latest => "latest",
        };
        write!(f, "{}", str_val)
    }
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub enum PinnedApiLinkPolicyTypeEnum {
    #[default]
    #[serde(rename = "pinned")]
    Pinned,
}
impl std::fmt::Display for PinnedApiLinkPolicyTypeEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            PinnedApiLinkPolicyTypeEnum::Pinned => "pinned",
        };
        write!(f, "{}", str_val)
    }
}
