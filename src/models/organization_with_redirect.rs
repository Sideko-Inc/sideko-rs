#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct OrganizationWithRedirect {
    #[cfg_attr(feature = "cli", arg(id = "organization", long = "organization"))]
    #[cfg_attr(
        feature = "cli",
        arg(value_parser = crate::core::clap::parse_json::<crate::models::Organization>)
    )]
    pub organization: crate::models::Organization,
    #[cfg_attr(feature = "cli", arg(id = "redirect-to", long = "redirect-to"))]
    pub redirect_to: String,
}
