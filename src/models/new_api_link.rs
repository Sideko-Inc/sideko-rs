#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct NewApiLink {
    #[cfg_attr(
        feature = "cli",
        arg(id = "build-request-enabled", long = "build-request-enabled")
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_request_enabled: Option<bool>,
    #[cfg_attr(feature = "cli", arg(id = "doc-version-id", long = "doc-version-id"))]
    pub doc_version_id: String,
    #[cfg_attr(feature = "cli", arg(id = "group-id", long = "group-id"))]
    pub group_id: String,
    #[cfg_attr(
        feature = "cli",
        arg(id = "include-mock-server", long = "include-mock-server")
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_mock_server: Option<bool>,
    #[cfg_attr(feature = "cli", arg(id = "nav-label", long = "nav-label"))]
    pub nav_label: String,
    #[cfg_attr(feature = "cli", arg(id = "policy", long = "policy"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_json::<crate::models::NewApiLinkPolicy>
        )
    )]
    pub policy: crate::models::NewApiLinkPolicy,
    #[cfg_attr(feature = "cli", arg(id = "slug", long = "slug"))]
    pub slug: String,
}
