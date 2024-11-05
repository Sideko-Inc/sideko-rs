#[derive(Clone, Debug)]
pub struct UpdateClient {
    base_client: crate::core::base_client::BaseClient,
}
impl UpdateClient {
    pub(crate) fn new(base_client: crate::core::base_client::BaseClient) -> Self {
        Self { base_client }
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
}
