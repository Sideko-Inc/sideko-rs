#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct InitRequest {
    #[cfg_attr(feature = "cli", command(flatten))]
    pub data: crate::models::InitSdkConfig,
}
#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct SyncRequest {
    #[cfg_attr(feature = "cli", command(flatten))]
    pub data: crate::models::SyncSdkConfig,
}
