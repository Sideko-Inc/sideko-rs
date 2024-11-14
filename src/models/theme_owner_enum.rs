#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
#[cfg_attr(feature = "cli", derive(clap::ValueEnum))]
pub enum ThemeOwnerEnum {
    #[default]
    #[cfg_attr(feature = "cli", value(name = "default"))]
    #[serde(rename = "default")]
    Default,
    #[cfg_attr(feature = "cli", value(name = "organization"))]
    #[serde(rename = "organization")]
    Organization,
    #[cfg_attr(feature = "cli", value(name = "self"))]
    #[serde(rename = "self")]
    _Self,
}
impl std::fmt::Display for ThemeOwnerEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            ThemeOwnerEnum::Default => "default",
            ThemeOwnerEnum::Organization => "organization",
            ThemeOwnerEnum::_Self => "self",
        };
        write!(f, "{}", str_val)
    }
}
