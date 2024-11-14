#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct Pagination {
    #[cfg_attr(feature = "cli", arg(id = "page", long = "page"))]
    pub page: i64,
    #[cfg_attr(feature = "cli", arg(id = "page-count", long = "page-count"))]
    pub page_count: i64,
    #[cfg_attr(feature = "cli", arg(id = "page-limit", long = "page-limit"))]
    pub page_limit: i64,
    #[cfg_attr(feature = "cli", arg(id = "total-count", long = "total-count"))]
    pub total_count: i64,
}
