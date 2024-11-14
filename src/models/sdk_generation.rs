#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct SdkGeneration {
    #[cfg_attr(feature = "cli", arg(id = "api-version-id", long = "api-version-id"))]
    pub api_version_id: String,
    #[cfg_attr(feature = "cli", arg(id = "created-at", long = "created-at"))]
    pub created_at: String,
    #[cfg_attr(feature = "cli", arg(id = "language", long = "language"))]
    pub language: crate::models::SdkLanguageEnum,
    #[cfg_attr(feature = "cli", arg(id = "name", long = "name"))]
    pub name: String,
    #[cfg_attr(feature = "cli", arg(id = "successful", long = "successful"))]
    pub successful: bool,
    #[cfg_attr(feature = "cli", arg(id = "version", long = "version"))]
    pub version: String,
}
