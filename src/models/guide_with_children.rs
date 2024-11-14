#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct GuideWithChildren {
    #[cfg_attr(feature = "cli", arg(id = "children", long = "children"))]
    #[cfg_attr(
        feature = "cli",
        arg(value_parser = crate::core::clap::parse_json::<Box<GuideWithChildren>>)
    )]
    #[cfg_attr(feature = "cli", arg(num_args = 0.., value_delimiter = ' '))]
    pub children: Vec<Box<GuideWithChildren>>,
    #[cfg_attr(feature = "cli", arg(id = "created-at", long = "created-at"))]
    pub created_at: String,
    #[cfg_attr(feature = "cli", arg(id = "id", long = "id"))]
    pub id: String,
    #[cfg_attr(feature = "cli", arg(id = "is-parent", long = "is-parent"))]
    pub is_parent: bool,
    #[cfg_attr(feature = "cli", arg(id = "nav-label", long = "nav-label"))]
    pub nav_label: String,
    #[cfg_attr(feature = "cli", arg(id = "order", long = "order"))]
    pub order: i64,
    #[cfg_attr(feature = "cli", arg(id = "parent-id", long = "parent-id"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    #[cfg_attr(feature = "cli", arg(id = "slug", long = "slug"))]
    pub slug: String,
}
