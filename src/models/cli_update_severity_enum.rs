#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
#[cfg_attr(feature = "cli", derive(clap::ValueEnum))]
pub enum CliUpdateSeverityEnum {
    #[default]
    #[cfg_attr(feature = "cli", value(name = "info"))]
    #[serde(rename = "info")]
    Info,
    #[cfg_attr(feature = "cli", value(name = "required"))]
    #[serde(rename = "required")]
    Required,
    #[cfg_attr(feature = "cli", value(name = "suggested"))]
    #[serde(rename = "suggested")]
    Suggested,
}
impl std::fmt::Display for CliUpdateSeverityEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            CliUpdateSeverityEnum::Info => "info",
            CliUpdateSeverityEnum::Required => "required",
            CliUpdateSeverityEnum::Suggested => "suggested",
        };
        write!(f, "{}", str_val)
    }
}
