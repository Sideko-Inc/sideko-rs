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
            .build_url(
                &format!(
                    "/role/{}", crate ::core::params::format_string_param(& request.id)
                ),
            );
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
    pub async fn list(
        &self,
        request: super::request_types::ListRequest,
    ) -> crate::SdkResult<Vec<crate::models::Role>> {
        let url = self.base_client.build_url("/role");
        let mut builder = reqwest::Client::default().get(&url);
        let mut queries = crate::core::params::QueryParams::default();
        queries.add_option("object_id", &request.object_id, false);
        queries.add_option("object_type", &request.object_type, false);
        queries.add_option("user_id", &request.user_id, false);
        builder = builder.query(&queries.params);
        builder = builder.header("x-sideko-sdk-language", "rust");
        builder = self
            .base_client
            .apply_auths_to_builder(builder, &["ApiKeyAuth", "CookieAuth"]);
        let mut response = builder.send().await?;
        response = self.base_client.error_for_status(response).await?;
        crate::core::response::process_json::<Vec<crate::models::Role>>(response).await
    }
    /// no description available
    pub async fn create(
        &self,
        request: super::request_types::CreateRequest,
    ) -> crate::SdkResult<Vec<crate::models::Role>> {
        let url = self.base_client.build_url("/role");
        let mut builder = reqwest::Client::default().post(&url);
        builder = builder.header("x-sideko-sdk-language", "rust");
        builder = builder.header("content-type", "application/json");
        builder = builder.json(&request.data);
        builder = self
            .base_client
            .apply_auths_to_builder(builder, &["ApiKeyAuth", "CookieAuth"]);
        let mut response = builder.send().await?;
        response = self.base_client.error_for_status(response).await?;
        crate::core::response::process_json::<Vec<crate::models::Role>>(response).await
    }
}
