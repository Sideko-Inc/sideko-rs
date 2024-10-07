#[derive(Clone, Debug)]
pub struct RoleClient {
    base_client: crate::core::base_client::BaseClient,
}
impl RoleClient {
    pub(crate) fn new(base_client: crate::core::base_client::BaseClient) -> Self {
        Self { base_client }
    }
    /// no description available
    pub async fn delete(
        &self,
        request: super::request_types::DeleteRequest,
    ) -> crate::SdkResult<()> {
        let url = self
            .base_client
            .build_url(&format!("/api/{}/role/{}", & request.id, & request.user_id));
        let mut builder = reqwest::Client::default().delete(&url);
        builder = builder.header("x-sideko-sdk-language", "rust");
        builder = self
            .base_client
            .apply_auths_to_builder(builder, &["ApiKeyAuth", "CookieAuth"]);
        let response = builder.send().await?;
        self.base_client.error_for_status(response).await?;
        Ok(())
    }
    /// no description available
    pub async fn create(
        &self,
        request: super::request_types::CreateRequest,
    ) -> crate::SdkResult<()> {
        let url = self.base_client.build_url(&format!("/api/{}/role", & request.id));
        let mut builder = reqwest::Client::default().post(&url);
        builder = builder.header("x-sideko-sdk-language", "rust");
        builder = builder.header("content-type", "application/json");
        builder = builder.json(&request.data);
        builder = self
            .base_client
            .apply_auths_to_builder(builder, &["ApiKeyAuth", "CookieAuth"]);
        let response = builder.send().await?;
        self.base_client.error_for_status(response).await?;
        Ok(())
    }
}
