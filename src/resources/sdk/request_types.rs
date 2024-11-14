#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct ListRequest {
    #[cfg_attr(feature = "cli", arg(id = "api-name", long = "api-name"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_name: Option<String>,
    #[cfg_attr(feature = "cli", arg(id = "successful", long = "successful"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub successful: Option<bool>,
}
#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct GenerateRequest {
    #[cfg_attr(feature = "cli", command(flatten))]
    pub data: crate::models::NewSdk,
}
#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct UpdateRequest {
    #[cfg_attr(feature = "cli", command(flatten))]
    pub data: crate::models::UpdateSdk,
}
#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct GenerateStatelessRequest {
    #[cfg_attr(feature = "cli", command(flatten))]
    pub data: crate::models::NewStatelessSdk,
}
