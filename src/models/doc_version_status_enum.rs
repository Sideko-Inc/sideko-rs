#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
#[cfg_attr(feature = "cli", derive(clap::ValueEnum))]
pub enum DocVersionStatusEnum {
    #[default]
    #[cfg_attr(feature = "cli", value(name = "Draft"))]
    #[serde(rename = "Draft")]
    Draft,
    #[cfg_attr(feature = "cli", value(name = "Published"))]
    #[serde(rename = "Published")]
    Published,
    #[cfg_attr(feature = "cli", value(name = "Publishing"))]
    #[serde(rename = "Publishing")]
    Publishing,
}
impl std::fmt::Display for DocVersionStatusEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            DocVersionStatusEnum::Draft => "Draft",
            DocVersionStatusEnum::Published => "Published",
            DocVersionStatusEnum::Publishing => "Publishing",
        };
        write!(f, "{}", str_val)
    }
}
