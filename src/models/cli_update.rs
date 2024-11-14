#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct CliUpdate {
    #[cfg_attr(feature = "cli", arg(id = "message", long = "message"))]
    pub message: String,
    #[cfg_attr(feature = "cli", arg(id = "severity", long = "severity"))]
    pub severity: crate::models::CliUpdateSeverityEnum,
}
