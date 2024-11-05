#[derive(Clone, Debug)]
pub struct GenerateSdkClient {
    base_client: crate::core::base_client::BaseClient,
}
impl GenerateSdkClient {
    pub(crate) fn new(base_client: crate::core::base_client::BaseClient) -> Self {
        Self { base_client }
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
