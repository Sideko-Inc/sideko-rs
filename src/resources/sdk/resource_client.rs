#[derive(Clone, Debug)]
pub struct SdkClient {
    base_client: crate::core::base_client::BaseClient,
}
impl SdkClient {
    pub(crate) fn new(base_client: crate::core::base_client::BaseClient) -> Self {
        Self { base_client }
    }
    pub fn config(
        &self,
    ) -> crate::resources::sdk::config::resource_client::ConfigClient {
        crate::resources::sdk::config::resource_client::ConfigClient::new(
            self.base_client.clone(),
        )
    }
    /// no description available
    pub async fn list(
        &self,
        request: super::request_types::ListRequest,
    ) -> crate::SdkResult<Vec<crate::models::SdkGeneration>> {
        let url = self.base_client.build_url("/sdk");
        let mut builder = reqwest::Client::default().get(&url);
        let mut queries = crate::core::params::QueryParams::default();
        queries.add_option("api_name", &request.api_name, false);
        queries.add_option("successful", &request.successful, false);
        builder = builder.query(&queries.params);
        builder = builder.header("x-sideko-sdk-language", "rust");
        builder = self
            .base_client
            .apply_auths_to_builder(builder, &["ApiKeyAuth", "CookieAuth"]);
        let mut response = builder.send().await?;
        response = self.base_client.error_for_status(response).await?;
        crate::core::response::process_json::<
            Vec<crate::models::SdkGeneration>,
        >(response)
            .await
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
            form_data = form_data
                .part("api_version", reqwest::multipart::Part::text(val.to_string()));
        }
        form_data = form_data
            .part("config", reqwest::multipart::Part::from(&request.data.config));
        form_data = form_data
            .part(
                "language",
                reqwest::multipart::Part::text(request.data.language.to_string()),
            );
        if let Some(val) = &request.data.sdk_version {
            form_data = form_data
                .part("sdk_version", reqwest::multipart::Part::text(val.to_string()));
        }
        builder = builder.multipart(form_data);
        builder = self
            .base_client
            .apply_auths_to_builder(builder, &["ApiKeyAuth", "CookieAuth"]);
        let mut response = builder.send().await?;
        response = self.base_client.error_for_status(response).await?;
        Ok(crate::BinaryResponse::new(response).await)
    }
    /// no description available
    pub async fn update(
        &self,
        request: super::request_types::UpdateRequest,
    ) -> crate::SdkResult<String> {
        let url = self.base_client.build_url("/sdk/update");
        let mut builder = reqwest::Client::default().post(&url);
        builder = builder.header("x-sideko-sdk-language", "rust");
        let mut form_data = reqwest::multipart::Form::new();
        if let Some(val) = &request.data.api_version {
            form_data = form_data
                .part("api_version", reqwest::multipart::Part::text(val.to_string()));
        }
        form_data = form_data
            .part("config", reqwest::multipart::Part::from(&request.data.config));
        form_data = form_data
            .part(
                "prev_sdk_git",
                reqwest::multipart::Part::from(&request.data.prev_sdk_git),
            );
        form_data = form_data
            .part(
                "prev_sdk_id",
                reqwest::multipart::Part::text(request.data.prev_sdk_id.to_string()),
            );
        form_data = form_data
            .part(
                "sdk_version",
                reqwest::multipart::Part::text(request.data.sdk_version.to_string()),
            );
        builder = builder.multipart(form_data);
        builder = self
            .base_client
            .apply_auths_to_builder(builder, &["ApiKeyAuth", "CookieAuth"]);
        let mut response = builder.send().await?;
        response = self.base_client.error_for_status(response).await?;
        Ok(response.text().await.unwrap_or_default())
    }
    /// no description available
    pub async fn generate_stateless(
        &self,
        request: super::request_types::GenerateStatelessRequest,
    ) -> crate::SdkResult<crate::BinaryResponse> {
        let url = self.base_client.build_url("/stateless/generate_sdk");
        let mut builder = reqwest::Client::default().post(&url);
        builder = builder.header("x-sideko-sdk-language", "rust");
        let mut form_data = reqwest::multipart::Form::new();
        if let Some(val) = &request.data.base_url {
            form_data = form_data
                .part("base_url", reqwest::multipart::Part::text(val.to_string()));
        }
        form_data = form_data
            .part(
                "language",
                reqwest::multipart::Part::text(request.data.language.to_string()),
            );
        form_data = form_data
            .part("openapi", reqwest::multipart::Part::from(&request.data.openapi));
        if let Some(val) = &request.data.package_name {
            form_data = form_data
                .part("package_name", reqwest::multipart::Part::text(val.to_string()));
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
