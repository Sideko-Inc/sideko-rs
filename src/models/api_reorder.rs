#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct ApiReorder {
    #[cfg_attr(feature = "cli", arg(id = "doc-version-id", long = "doc-version-id"))]
    pub doc_version_id: String,
    #[cfg_attr(feature = "cli", arg(id = "groups", long = "groups"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_json::<crate::models::ApiLinkGroupReorder>
        )
    )]
    #[cfg_attr(feature = "cli", arg(num_args = 0.., value_delimiter = ' '))]
    pub groups: Vec<crate::models::ApiLinkGroupReorder>,
    #[cfg_attr(feature = "cli", arg(id = "links", long = "links"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_json::<crate::models::ApiLinkReorder>
        )
    )]
    #[cfg_attr(feature = "cli", arg(num_args = 0.., value_delimiter = ' '))]
    pub links: Vec<crate::models::ApiLinkReorder>,
}
