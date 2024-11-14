#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct OrganizationFeatures {
    #[cfg_attr(feature = "cli", arg(id = "max-api-projects", long = "max-api-projects"))]
    pub max_api_projects: i64,
    #[cfg_attr(feature = "cli", arg(id = "max-doc-projects", long = "max-doc-projects"))]
    pub max_doc_projects: i64,
    #[cfg_attr(feature = "cli", arg(id = "max-mock-servers", long = "max-mock-servers"))]
    pub max_mock_servers: i64,
    #[cfg_attr(feature = "cli", arg(id = "max-teamates", long = "max-teamates"))]
    pub max_teamates: i64,
}
