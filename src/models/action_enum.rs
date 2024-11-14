#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
#[cfg_attr(feature = "cli", derive(clap::ValueEnum))]
pub enum ActionEnum {
    #[default]
    #[cfg_attr(feature = "cli", value(name = "api_project_delete"))]
    #[serde(rename = "api_project_delete")]
    ApiProjectDelete,
    #[cfg_attr(feature = "cli", value(name = "api_project_read"))]
    #[serde(rename = "api_project_read")]
    ApiProjectRead,
    #[cfg_attr(feature = "cli", value(name = "api_project_update"))]
    #[serde(rename = "api_project_update")]
    ApiProjectUpdate,
    #[cfg_attr(feature = "cli", value(name = "api_project_version_create"))]
    #[serde(rename = "api_project_version_create")]
    ApiProjectVersionCreate,
    #[cfg_attr(feature = "cli", value(name = "api_project_version_delete"))]
    #[serde(rename = "api_project_version_delete")]
    ApiProjectVersionDelete,
    #[cfg_attr(feature = "cli", value(name = "api_project_version_read"))]
    #[serde(rename = "api_project_version_read")]
    ApiProjectVersionRead,
    #[cfg_attr(feature = "cli", value(name = "api_project_version_update"))]
    #[serde(rename = "api_project_version_update")]
    ApiProjectVersionUpdate,
    #[cfg_attr(feature = "cli", value(name = "audit_log_read"))]
    #[serde(rename = "audit_log_read")]
    AuditLogRead,
    #[cfg_attr(feature = "cli", value(name = "doc_project_delete"))]
    #[serde(rename = "doc_project_delete")]
    DocProjectDelete,
    #[cfg_attr(feature = "cli", value(name = "doc_project_publish_preview"))]
    #[serde(rename = "doc_project_publish_preview")]
    DocProjectPublishPreview,
    #[cfg_attr(feature = "cli", value(name = "doc_project_publish_production"))]
    #[serde(rename = "doc_project_publish_production")]
    DocProjectPublishProduction,
    #[cfg_attr(feature = "cli", value(name = "doc_project_read"))]
    #[serde(rename = "doc_project_read")]
    DocProjectRead,
    #[cfg_attr(feature = "cli", value(name = "doc_project_update"))]
    #[serde(rename = "doc_project_update")]
    DocProjectUpdate,
    #[cfg_attr(feature = "cli", value(name = "doc_project_version_read"))]
    #[serde(rename = "doc_project_version_read")]
    DocProjectVersionRead,
    #[cfg_attr(feature = "cli", value(name = "doc_project_version_update"))]
    #[serde(rename = "doc_project_version_update")]
    DocProjectVersionUpdate,
    #[cfg_attr(feature = "cli", value(name = "organization_create_api_project"))]
    #[serde(rename = "organization_create_api_project")]
    OrganizationCreateApiProject,
    #[cfg_attr(feature = "cli", value(name = "organization_create_doc_project"))]
    #[serde(rename = "organization_create_doc_project")]
    OrganizationCreateDocProject,
    #[cfg_attr(feature = "cli", value(name = "organization_read_theme"))]
    #[serde(rename = "organization_read_theme")]
    OrganizationReadTheme,
    #[cfg_attr(feature = "cli", value(name = "organization_update_theme"))]
    #[serde(rename = "organization_update_theme")]
    OrganizationUpdateTheme,
    #[cfg_attr(feature = "cli", value(name = "role_create"))]
    #[serde(rename = "role_create")]
    RoleCreate,
    #[cfg_attr(feature = "cli", value(name = "role_delete"))]
    #[serde(rename = "role_delete")]
    RoleDelete,
    #[cfg_attr(feature = "cli", value(name = "role_read"))]
    #[serde(rename = "role_read")]
    RoleRead,
}
impl std::fmt::Display for ActionEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            ActionEnum::ApiProjectDelete => "api_project_delete",
            ActionEnum::ApiProjectRead => "api_project_read",
            ActionEnum::ApiProjectUpdate => "api_project_update",
            ActionEnum::ApiProjectVersionCreate => "api_project_version_create",
            ActionEnum::ApiProjectVersionDelete => "api_project_version_delete",
            ActionEnum::ApiProjectVersionRead => "api_project_version_read",
            ActionEnum::ApiProjectVersionUpdate => "api_project_version_update",
            ActionEnum::AuditLogRead => "audit_log_read",
            ActionEnum::DocProjectDelete => "doc_project_delete",
            ActionEnum::DocProjectPublishPreview => "doc_project_publish_preview",
            ActionEnum::DocProjectPublishProduction => "doc_project_publish_production",
            ActionEnum::DocProjectRead => "doc_project_read",
            ActionEnum::DocProjectUpdate => "doc_project_update",
            ActionEnum::DocProjectVersionRead => "doc_project_version_read",
            ActionEnum::DocProjectVersionUpdate => "doc_project_version_update",
            ActionEnum::OrganizationCreateApiProject => "organization_create_api_project",
            ActionEnum::OrganizationCreateDocProject => "organization_create_doc_project",
            ActionEnum::OrganizationReadTheme => "organization_read_theme",
            ActionEnum::OrganizationUpdateTheme => "organization_update_theme",
            ActionEnum::RoleCreate => "role_create",
            ActionEnum::RoleDelete => "role_delete",
            ActionEnum::RoleRead => "role_read",
        };
        write!(f, "{}", str_val)
    }
}
