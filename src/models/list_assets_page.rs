#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct ListAssetsPage {
    #[cfg_attr(feature = "cli", arg(id = "pagination", long = "pagination"))]
    #[cfg_attr(
        feature = "cli",
        arg(value_parser = crate::core::clap::parse_json::<crate::models::Pagination>)
    )]
    pub pagination: crate::models::Pagination,
    #[cfg_attr(feature = "cli", arg(id = "results", long = "results"))]
    #[cfg_attr(
        feature = "cli",
        arg(value_parser = crate::core::clap::parse_json::<crate::models::Asset>)
    )]
    #[cfg_attr(feature = "cli", arg(num_args = 0.., value_delimiter = ' '))]
    pub results: Vec<crate::models::Asset>,
}
