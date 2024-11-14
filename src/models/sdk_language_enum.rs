#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
#[cfg_attr(feature = "cli", derive(clap::ValueEnum))]
pub enum SdkLanguageEnum {
    #[default]
    #[cfg_attr(feature = "cli", value(name = "go"))]
    #[serde(rename = "go")]
    Go,
    #[cfg_attr(feature = "cli", value(name = "java"))]
    #[serde(rename = "java")]
    Java,
    #[cfg_attr(feature = "cli", value(name = "python"))]
    #[serde(rename = "python")]
    Python,
    #[cfg_attr(feature = "cli", value(name = "ruby"))]
    #[serde(rename = "ruby")]
    Ruby,
    #[cfg_attr(feature = "cli", value(name = "rust"))]
    #[serde(rename = "rust")]
    Rust,
    #[cfg_attr(feature = "cli", value(name = "typescript"))]
    #[serde(rename = "typescript")]
    Typescript,
}
impl std::fmt::Display for SdkLanguageEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            SdkLanguageEnum::Go => "go",
            SdkLanguageEnum::Java => "java",
            SdkLanguageEnum::Python => "python",
            SdkLanguageEnum::Ruby => "ruby",
            SdkLanguageEnum::Rust => "rust",
            SdkLanguageEnum::Typescript => "typescript",
        };
        write!(f, "{}", str_val)
    }
}
