#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct NewServiceAccount {
    #[cfg_attr(feature = "cli", arg(id = "name", long = "name"))]
    pub name: String,
    #[cfg_attr(feature = "cli", arg(id = "object-roles", long = "object-roles"))]
    #[cfg_attr(
        feature = "cli",
        arg(value_parser = crate::core::clap::parse_json::<crate::models::ObjectRole>)
    )]
    #[cfg_attr(feature = "cli", arg(num_args = 0.., value_delimiter = ' '))]
    pub object_roles: Vec<crate::models::ObjectRole>,
}
