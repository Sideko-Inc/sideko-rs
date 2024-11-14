#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct UserApiKey {
    #[cfg_attr(feature = "cli", arg(id = "api-key", long = "api-key"))]
    pub api_key: String,
    #[cfg_attr(feature = "cli", arg(id = "avatar-url", long = "avatar-url"))]
    pub avatar_url: String,
    #[cfg_attr(feature = "cli", arg(id = "created-at", long = "created-at"))]
    pub created_at: String,
    #[cfg_attr(feature = "cli", arg(id = "email", long = "email"))]
    pub email: String,
    #[cfg_attr(feature = "cli", arg(id = "first-name", long = "first-name"))]
    pub first_name: String,
    #[cfg_attr(feature = "cli", arg(id = "id", long = "id"))]
    pub id: String,
    #[cfg_attr(
        feature = "cli",
        arg(id = "is-service-account", long = "is-service-account")
    )]
    pub is_service_account: bool,
    #[cfg_attr(feature = "cli", arg(id = "last-name", long = "last-name"))]
    pub last_name: String,
}
