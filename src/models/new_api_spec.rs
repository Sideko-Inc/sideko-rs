#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct NewApiSpec {
    #[cfg_attr(
        feature = "cli",
        arg(id = "mock-server-enabled", long = "mock-server-enabled")
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mock_server_enabled: Option<bool>,
    #[cfg_attr(feature = "cli", arg(id = "notes", long = "notes"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[cfg_attr(feature = "cli", arg(id = "openapi", long = "openapi"))]
    #[cfg_attr(
        feature = "cli",
        arg(value_parser = crate::core::clap::parse_upload_file)
    )]
    pub openapi: crate::UploadFile,
    #[cfg_attr(feature = "cli", arg(id = "version", long = "version"))]
    #[cfg_attr(
        feature = "cli",
        arg(value_parser = crate::core::clap::parse_json::<crate::models::VersionOrBump>)
    )]
    pub version: crate::models::VersionOrBump,
}
