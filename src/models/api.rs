/// File Generated by Sideko (sideko.dev)
#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct Api {
    #[cfg_attr(feature = "cli", arg(id = "created-at", long = "created-at"))]
    pub created_at: String,
    #[cfg_attr(feature = "cli", arg(id = "id", long = "id"))]
    pub id: String,
}