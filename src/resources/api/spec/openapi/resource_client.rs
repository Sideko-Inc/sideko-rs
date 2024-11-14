#[derive(Clone, Debug)]
pub struct OpenapiClient {
    base_client: crate::core::base_client::BaseClient,
}
impl OpenapiClient {
    pub(crate) fn new(base_client: crate::core::base_client::BaseClient) -> Self {
        Self { base_client }
    }
    /// no description available
    pub async fn get_openapi(
        &self,
        request: super::request_types::GetOpenapiRequest,
    ) -> crate::SdkResult<crate::models::OpenApi> {
        let url = self
            .base_client
            .build_url(
                &format!(
                    "/api/{}/spec/{}/openapi", crate
                    ::core::params::format_string_param(& request.api_name), crate
                    ::core::params::format_string_param(& request.api_version)
                ),
            );
        let mut builder = reqwest::Client::default().get(&url);
        builder = builder.header("x-sideko-sdk-language", "rust");
        builder = self
            .base_client
            .apply_auths_to_builder(builder, &["ApiKeyAuth", "CookieAuth"]);
        let mut response = builder.send().await?;
        response = self.base_client.error_for_status(response).await?;
        crate::core::response::process_json::<crate::models::OpenApi>(response).await
    }
}
