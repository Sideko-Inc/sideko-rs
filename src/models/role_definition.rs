/// File Generated by Sideko (sideko.dev)
#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct RoleDefinition {
    #[cfg_attr(feature = "cli", arg(id = "actions", long = "actions"))]
    #[cfg_attr(feature = "cli", arg(num_args = 0.., value_delimiter = ' '))]
    pub actions: Vec<crate::models::ActionEnum>,
    #[cfg_attr(feature = "cli", arg(id = "display-name", long = "display-name"))]
    pub display_name: String,
    #[cfg_attr(feature = "cli", arg(id = "id", long = "id"))]
    pub id: crate::models::RoleDefinitionIdEnum,
}