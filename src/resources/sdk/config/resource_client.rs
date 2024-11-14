#[derive(Clone, Debug)]
pub struct ConfigClient {
    base_client: crate::core::base_client::BaseClient,
}
impl ConfigClient {
    pub(crate) fn new(base_client: crate::core::base_client::BaseClient) -> Self {
        Self { base_client }
    }
    /// Creates a sdk config with default configurations for the api/api version
    pub async fn init(
        &self,
        request: super::request_types::InitRequest,
    ) -> crate::SdkResult<crate::BinaryResponse> {
        let url = self.base_client.build_url("/sdk/config/init");
        let mut builder = reqwest::Client::default().post(&url);
        builder = builder.header("x-sideko-sdk-language", "rust");
        builder = builder.header("content-type", "application/json");
        builder = builder.json(&request.data);
        builder = self
            .base_client
            .apply_auths_to_builder(builder, &["ApiKeyAuth", "CookieAuth"]);
        let mut response = builder.send().await?;
        response = self.base_client.error_for_status(response).await?;
        Ok(crate::BinaryResponse::new(response).await)
    }
    /// Updates provided config with missing default configurations for the api version
    pub async fn sync(
        &self,
        request: super::request_types::SyncRequest,
    ) -> crate::SdkResult<crate::BinaryResponse> {
        let url = self.base_client.build_url("/sdk/config/sync");
        let mut builder = reqwest::Client::default().post(&url);
        builder = builder.header("x-sideko-sdk-language", "rust");
        let mut form_data = reqwest::multipart::Form::new();
        if let Some(val) = &request.data.api_version {
            form_data = form_data
                .part("api_version", reqwest::multipart::Part::text(val.to_string()));
        }
        form_data = form_data
            .part("config", reqwest::multipart::Part::from(&request.data.config));
        builder = builder.multipart(form_data);
        builder = self
            .base_client
            .apply_auths_to_builder(builder, &["ApiKeyAuth", "CookieAuth"]);
        let mut response = builder.send().await?;
        response = self.base_client.error_for_status(response).await?;
        Ok(crate::BinaryResponse::new(response).await)
    }
}
