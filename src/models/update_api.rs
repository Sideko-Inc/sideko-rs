/// File Generated by Sideko (sideko.dev)
#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct UpdateApi {
    #[cfg_attr(feature = "cli", arg(id = "name", long = "name"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
