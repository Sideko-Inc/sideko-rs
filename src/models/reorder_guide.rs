#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct ReorderGuide {
    #[cfg_attr(feature = "cli", arg(id = "id", long = "id"))]
    pub id: String,
    #[cfg_attr(feature = "cli", arg(id = "order", long = "order"))]
    pub order: i64,
    #[cfg_attr(feature = "cli", arg(id = "parent-id", long = "parent-id"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
}
