#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct GetStatsRequest {
    #[cfg_attr(feature = "cli", arg(id = "api-name", long = "api-name"))]
    pub api_name: String,
    #[cfg_attr(feature = "cli", arg(id = "api-version", long = "api-version"))]
    #[cfg_attr(
        feature = "cli",
        arg(value_parser = crate::core::clap::parse_json::<crate::models::ApiVersion>)
    )]
    pub api_version: crate::models::ApiVersion,
}
