/// File Generated by Sideko (sideko.dev)
#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct ApiMockServer {
    #[cfg_attr(feature = "cli", arg(id = "enabled", long = "enabled"))]
    pub enabled: bool,
    #[cfg_attr(feature = "cli", arg(id = "url", long = "url"))]
    pub url: String,
}