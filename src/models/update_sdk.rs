#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct UpdateSdk {
    #[cfg_attr(feature = "cli", arg(id = "api-version", long = "api-version"))]
    #[cfg_attr(
        feature = "cli",
        arg(value_parser = crate::core::clap::parse_json::<crate::models::ApiVersion>)
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_version: Option<crate::models::ApiVersion>,
    #[cfg_attr(feature = "cli", arg(id = "config", long = "config"))]
    #[cfg_attr(
        feature = "cli",
        arg(value_parser = crate::core::clap::parse_upload_file)
    )]
    pub config: crate::UploadFile,
    #[cfg_attr(feature = "cli", arg(id = "prev-sdk-git", long = "prev-sdk-git"))]
    #[cfg_attr(
        feature = "cli",
        arg(value_parser = crate::core::clap::parse_upload_file)
    )]
    pub prev_sdk_git: crate::UploadFile,
    #[cfg_attr(feature = "cli", arg(id = "prev-sdk-id", long = "prev-sdk-id"))]
    pub prev_sdk_id: String,
    #[cfg_attr(feature = "cli", arg(id = "sdk-version", long = "sdk-version"))]
    #[cfg_attr(
        feature = "cli",
        arg(value_parser = crate::core::clap::parse_json::<crate::models::VersionOrBump>)
    )]
    pub sdk_version: crate::models::VersionOrBump,
}
