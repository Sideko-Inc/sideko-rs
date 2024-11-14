#[derive(Clone, Debug)]
pub struct ApiClient {
    base_client: crate::core::base_client::BaseClient,
}
impl ApiClient {
    pub(crate) fn new(base_client: crate::core::base_client::BaseClient) -> Self {
        Self { base_client }
    }
    pub fn spec(&self) -> crate::resources::api::spec::resource_client::SpecClient {
        crate::resources::api::spec::resource_client::SpecClient::new(
            self.base_client.clone(),
        )
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
                    "/api/{}", crate ::core::params::format_string_param(& request
                    .api_name)
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
    pub async fn list(&self) -> crate::SdkResult<Vec<crate::models::Api>> {
        let url = self.base_client.build_url("/api");
        let mut builder = reqwest::Client::default().get(&url);
        builder = builder.header("x-sideko-sdk-language", "rust");
        builder = self
            .base_client
            .apply_auths_to_builder(builder, &["ApiKeyAuth", "CookieAuth"]);
        let mut response = builder.send().await?;
        response = self.base_client.error_for_status(response).await?;
        crate::core::response::process_json::<Vec<crate::models::Api>>(response).await
    }
    /// no description available
    pub async fn get(
        &self,
        request: super::request_types::GetRequest,
    ) -> crate::SdkResult<crate::models::Api> {
        let url = self
            .base_client
            .build_url(
                &format!(
                    "/api/{}", crate ::core::params::format_string_param(& request
                    .api_name)
                ),
            );
        let mut builder = reqwest::Client::default().get(&url);
        builder = builder.header("x-sideko-sdk-language", "rust");
        builder = self
            .base_client
            .apply_auths_to_builder(builder, &["ApiKeyAuth", "CookieAuth"]);
        let mut response = builder.send().await?;
        response = self.base_client.error_for_status(response).await?;
        crate::core::response::process_json::<crate::models::Api>(response).await
    }
    /// no description available
    pub async fn patch(
        &self,
        request: super::request_types::PatchRequest,
    ) -> crate::SdkResult<crate::models::Api> {
        let url = self
            .base_client
            .build_url(
                &format!(
                    "/api/{}", crate ::core::params::format_string_param(& request
                    .api_name)
                ),
            );
        let mut builder = reqwest::Client::default().patch(&url);
        builder = builder.header("x-sideko-sdk-language", "rust");
        builder = builder.header("content-type", "application/json");
        builder = builder.json(&request.data);
        builder = self
            .base_client
            .apply_auths_to_builder(builder, &["ApiKeyAuth", "CookieAuth"]);
        let mut response = builder.send().await?;
        response = self.base_client.error_for_status(response).await?;
        crate::core::response::process_json::<crate::models::Api>(response).await
    }
    /// no description available
    pub async fn create(
        &self,
        request: super::request_types::CreateRequest,
    ) -> crate::SdkResult<crate::models::Api> {
        let url = self.base_client.build_url("/api");
        let mut builder = reqwest::Client::default().post(&url);
        builder = builder.header("x-sideko-sdk-language", "rust");
        builder = builder.header("content-type", "application/json");
        builder = builder.json(&request.data);
        builder = self
            .base_client
            .apply_auths_to_builder(builder, &["ApiKeyAuth", "CookieAuth"]);
        let mut response = builder.send().await?;
        response = self.base_client.error_for_status(response).await?;
        crate::core::response::process_json::<crate::models::Api>(response).await
    }
}
