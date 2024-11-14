#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct DeleteRequest {
    #[cfg_attr(feature = "cli", arg(id = "id", long = "id"))]
    pub id: String,
}
#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct ListRequest {
    #[cfg_attr(feature = "cli", arg(id = "object-id", long = "object-id"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_id: Option<String>,
    #[cfg_attr(feature = "cli", arg(id = "object-type", long = "object-type"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_type: Option<crate::models::ObjectTypeEnum>,
    #[cfg_attr(feature = "cli", arg(id = "user-id", long = "user-id"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}
#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct CreateRequest {
    #[cfg_attr(feature = "cli", command(flatten))]
    pub data: crate::models::NewRole,
}
