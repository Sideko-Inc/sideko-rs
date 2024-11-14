#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct ApiLinkApiVersion {
    #[cfg_attr(feature = "cli", arg(id = "api-id", long = "api-id"))]
    pub api_id: String,
    #[cfg_attr(feature = "cli", arg(id = "api-name", long = "api-name"))]
    pub api_name: String,
    #[cfg_attr(feature = "cli", arg(id = "id", long = "id"))]
    pub id: String,
    #[cfg_attr(feature = "cli", arg(id = "version", long = "version"))]
    pub version: String,
}
