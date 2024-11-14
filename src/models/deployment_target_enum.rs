#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
#[cfg_attr(feature = "cli", derive(clap::ValueEnum))]
pub enum DeploymentTargetEnum {
    #[default]
    #[cfg_attr(feature = "cli", value(name = "Preview"))]
    #[serde(rename = "Preview")]
    Preview,
    #[cfg_attr(feature = "cli", value(name = "Production"))]
    #[serde(rename = "Production")]
    Production,
}
impl std::fmt::Display for DeploymentTargetEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            DeploymentTargetEnum::Preview => "Preview",
            DeploymentTargetEnum::Production => "Production",
        };
        write!(f, "{}", str_val)
    }
}
