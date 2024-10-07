#[derive(Clone, Debug)]
pub struct MembersClient {
    base_client: crate::core::base_client::BaseClient,
}
impl MembersClient {
    pub(crate) fn new(base_client: crate::core::base_client::BaseClient) -> Self {
        Self { base_client }
    }
    /// no description available
    pub async fn list(
        &self,
        request: super::request_types::ListRequest,
    ) -> crate::SdkResult<Vec<crate::models::ProjectMember>> {
        let url = self.base_client.build_url(&format!("/api/{}/members", & request.id));
        let mut builder = reqwest::Client::default().get(&url);
        builder = builder.header("x-sideko-sdk-language", "rust");
        builder = self
            .base_client
            .apply_auths_to_builder(builder, &["ApiKeyAuth", "CookieAuth"]);
        let mut response = builder.send().await?;
        response = self.base_client.error_for_status(response).await?;
        crate::core::response::process_json::<
            Vec<crate::models::ProjectMember>,
        >(response)
            .await
    }
}
