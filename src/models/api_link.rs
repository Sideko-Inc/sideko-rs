#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct ApiLink {
    #[cfg_attr(feature = "cli", arg(id = "api-version", long = "api-version"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_json::<crate::models::ApiLinkApiVersion>
        )
    )]
    pub api_version: crate::models::ApiLinkApiVersion,
    #[cfg_attr(
        feature = "cli",
        arg(id = "build-request-enabled", long = "build-request-enabled")
    )]
    pub build_request_enabled: bool,
    #[cfg_attr(feature = "cli", arg(id = "created-at", long = "created-at"))]
    pub created_at: String,
    #[cfg_attr(feature = "cli", arg(id = "doc-version", long = "doc-version"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_json::<crate::models::ApiLinkDocVersion>
        )
    )]
    pub doc_version: crate::models::ApiLinkDocVersion,
    #[cfg_attr(feature = "cli", arg(id = "group-id", long = "group-id"))]
    pub group_id: String,
    #[cfg_attr(feature = "cli", arg(id = "id", long = "id"))]
    pub id: String,
    #[cfg_attr(
        feature = "cli",
        arg(id = "include-mock-server", long = "include-mock-server")
    )]
    pub include_mock_server: bool,
    #[cfg_attr(feature = "cli", arg(id = "nav-label", long = "nav-label"))]
    pub nav_label: String,
    #[cfg_attr(feature = "cli", arg(id = "order", long = "order"))]
    pub order: i64,
    #[cfg_attr(feature = "cli", arg(id = "policy", long = "policy"))]
    pub policy: crate::models::ApiLinkPolicyEnum,
    #[cfg_attr(feature = "cli", arg(id = "slug", long = "slug"))]
    pub slug: String,
}
