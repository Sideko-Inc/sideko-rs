/// File Generated by Sideko (sideko.dev)
#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct Error {
    #[cfg_attr(feature = "cli", arg(id = "error", long = "error"))]
    pub error: crate::models::ErrorErrorEnum,
    #[cfg_attr(feature = "cli", arg(id = "message", long = "message"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
