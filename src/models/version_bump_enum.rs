#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
#[cfg_attr(feature = "cli", derive(clap::ValueEnum))]
pub enum VersionBumpEnum {
    #[default]
    #[cfg_attr(feature = "cli", value(name = "major"))]
    #[serde(rename = "major")]
    Major,
    #[cfg_attr(feature = "cli", value(name = "minor"))]
    #[serde(rename = "minor")]
    Minor,
    #[cfg_attr(feature = "cli", value(name = "patch"))]
    #[serde(rename = "patch")]
    Patch,
    #[cfg_attr(feature = "cli", value(name = "rc"))]
    #[serde(rename = "rc")]
    Rc,
}
impl std::fmt::Display for VersionBumpEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            VersionBumpEnum::Major => "major",
            VersionBumpEnum::Minor => "minor",
            VersionBumpEnum::Patch => "patch",
            VersionBumpEnum::Rc => "rc",
        };
        write!(f, "{}", str_val)
    }
}
