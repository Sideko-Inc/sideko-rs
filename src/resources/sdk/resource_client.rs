#[derive(Clone, Debug)]
pub struct SdkClient {
    base_client: crate::core::base_client::BaseClient,
}
impl SdkClient {
    pub(crate) fn new(base_client: crate::core::base_client::BaseClient) -> Self {
        Self { base_client }
    }
    pub fn config(&self) -> crate::resources::sdk::config::resource_client::ConfigClient {
        crate::resources::sdk::config::resource_client::ConfigClient::new(self.base_client.clone())
    }
    pub fn update(&self) -> crate::resources::sdk::update::resource_client::UpdateClient {
        crate::resources::sdk::update::resource_client::UpdateClient::new(self.base_client.clone())
    }
    /// no description available
    pub async fn list(
        &self,
        request: super::request_types::ListRequest,
    ) -> crate::SdkResult<Vec<crate::models::SdkGeneration>> {
        let url = self.base_client.build_url("/sdk");
        let mut builder = reqwest::Client::default().get(&url);
        let mut queries = crate::core::params::QueryParams::default();
        queries.add_option("api", &request.api, false);
        queries.add_option("successful", &request.successful, false);
        builder = builder.query(&queries.params);
        builder = builder.header("x-sideko-sdk-language", "rust");
        builder = self
            .base_client
            .apply_auths_to_builder(builder, &["ApiKeyAuth", "CookieAuth"]);
        let mut response = builder.send().await?;
        response = self.base_client.error_for_status(response).await?;
        crate::core::response::process_json::<Vec<crate::models::SdkGeneration>>(response).await
    }
    /// no description available
    pub async fn generate(
        &self,
        request: super::request_types::GenerateRequest,
    ) -> crate::SdkResult<crate::BinaryResponse> {
        let url = self.base_client.build_url("/sdk");
        let mut builder = reqwest::Client::default().post(&url);
        builder = builder.header("x-sideko-sdk-language", "rust");
        let mut form_data = reqwest::multipart::Form::new();
        if let Some(val) = &request.data.api_version {
            form_data = form_data.part(
                "api_version",
                reqwest::multipart::Part::text(crate::core::params::format_string_param(val)),
            );
        }
        form_data = form_data.part(
            "config",
            reqwest::multipart::Part::from(&request.data.config),
        );
        form_data = form_data.part(
            "language",
            reqwest::multipart::Part::text(request.data.language.to_string()),
        );
        if let Some(val) = &request.data.sdk_version {
            form_data = form_data.part(
                "sdk_version",
                reqwest::multipart::Part::text(val.to_string()),
            );
        }
        builder = builder.multipart(form_data);
        builder = self
            .base_client
            .apply_auths_to_builder(builder, &["ApiKeyAuth", "CookieAuth"]);
        let mut response = builder.send().await?;
        response = self.base_client.error_for_status(response).await?;
        Ok(crate::BinaryResponse::new(response).await)
    }
}
