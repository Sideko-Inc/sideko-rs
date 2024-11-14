#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
#[cfg_attr(feature = "cli", derive(clap::ValueEnum))]
pub enum ValidationSeverityEnum {
    #[default]
    #[cfg_attr(feature = "cli", value(name = "error"))]
    #[serde(rename = "error")]
    Error,
    #[cfg_attr(feature = "cli", value(name = "info"))]
    #[serde(rename = "info")]
    Info,
    #[cfg_attr(feature = "cli", value(name = "warning"))]
    #[serde(rename = "warning")]
    Warning,
}
impl std::fmt::Display for ValidationSeverityEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            ValidationSeverityEnum::Error => "error",
            ValidationSeverityEnum::Info => "info",
            ValidationSeverityEnum::Warning => "warning",
        };
        write!(f, "{}", str_val)
    }
}
