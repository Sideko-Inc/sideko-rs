#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct ObjectRole {
    #[cfg_attr(feature = "cli", arg(id = "object-id", long = "object-id"))]
    pub object_id: String,
    #[cfg_attr(feature = "cli", arg(id = "object-type", long = "object-type"))]
    pub object_type: crate::models::ObjectTypeEnum,
    #[cfg_attr(
        feature = "cli",
        arg(id = "role-definition-id", long = "role-definition-id")
    )]
    pub role_definition_id: crate::models::RoleDefinitionIdEnum,
}
