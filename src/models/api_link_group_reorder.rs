#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct ApiLinkGroupReorder {
    #[cfg_attr(feature = "cli", arg(id = "id", long = "id"))]
    pub id: String,
    #[cfg_attr(feature = "cli", arg(id = "order", long = "order"))]
    pub order: i64,
}
