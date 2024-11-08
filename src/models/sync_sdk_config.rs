/// File Generated by Sideko (sideko.dev)
#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct SyncSdkConfig {
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
}
