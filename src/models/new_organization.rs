#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct NewOrganization {
    #[cfg_attr(feature = "cli", arg(id = "name", long = "name"))]
    pub name: String,
    #[cfg_attr(feature = "cli", arg(id = "subdomain", long = "subdomain"))]
    pub subdomain: String,
}
