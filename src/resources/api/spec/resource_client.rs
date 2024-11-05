#[derive(Clone, Debug)]
pub struct SpecClient {
    base_client: crate::core::base_client::BaseClient,
}
impl SpecClient {
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
                &format!("/api/{}/spec/{}", & request.api_name, & request.api_version),
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
    ) -> crate::SdkResult<Vec<crate::models::ApiSpec>> {
        let url = self
            .base_client
            .build_url(&format!("/api/{}/spec", & request.api_name));
        let mut builder = reqwest::Client::default().get(&url);
        builder = builder.header("x-sideko-sdk-language", "rust");
        builder = self
            .base_client
            .apply_auths_to_builder(builder, &["ApiKeyAuth", "CookieAuth"]);
        let mut response = builder.send().await?;
        response = self.base_client.error_for_status(response).await?;
        crate::core::response::process_json::<Vec<crate::models::ApiSpec>>(response)
            .await
    }
    /// no description available
    pub async fn get(
        &self,
        request: super::request_types::GetRequest,
    ) -> crate::SdkResult<crate::models::ApiSpec> {
        let url = self
            .base_client
            .build_url(
                &format!("/api/{}/spec/{}", & request.api_name, & request.api_version),
            );
        let mut builder = reqwest::Client::default().get(&url);
        builder = builder.header("x-sideko-sdk-language", "rust");
        builder = self
            .base_client
            .apply_auths_to_builder(builder, &["ApiKeyAuth", "CookieAuth"]);
        let mut response = builder.send().await?;
        response = self.base_client.error_for_status(response).await?;
        crate::core::response::process_json::<crate::models::ApiSpec>(response).await
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
                    "/api/{}/spec/{}/openapi", & request.api_name, & request.api_version
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
    /// no description available
    pub async fn get_stats(
        &self,
        request: super::request_types::GetStatsRequest,
    ) -> crate::SdkResult<crate::models::ApiSpecStats> {
        let url = self
            .base_client
            .build_url(
                &format!(
                    "/api/{}/spec/{}/stats", & request.api_name, & request.api_version
                ),
            );
        let mut builder = reqwest::Client::default().get(&url);
        builder = builder.header("x-sideko-sdk-language", "rust");
        builder = self
            .base_client
            .apply_auths_to_builder(builder, &["ApiKeyAuth", "CookieAuth"]);
        let mut response = builder.send().await?;
        response = self.base_client.error_for_status(response).await?;
        crate::core::response::process_json::<crate::models::ApiSpecStats>(response)
            .await
    }
    /// no description available
    pub async fn patch(
        &self,
        request: super::request_types::PatchRequest,
    ) -> crate::SdkResult<crate::models::ApiSpec> {
        let url = self
            .base_client
            .build_url(
                &format!("/api/{}/spec/{}", & request.api_name, & request.api_version),
            );
        let mut builder = reqwest::Client::default().patch(&url);
        builder = builder.header("x-sideko-sdk-language", "rust");
        let mut form_data = reqwest::multipart::Form::new();
        if let Some(val) = &request.data.mock_server_enabled {
            form_data = form_data
                .part(
                    "mock_server_enabled",
                    reqwest::multipart::Part::text(val.to_string()),
                );
        }
        if let Some(val) = &request.data.notes {
            form_data = form_data
                .part("notes", reqwest::multipart::Part::text(val.to_string()));
        }
        if let Some(val) = &request.data.openapi {
            form_data = form_data.part("openapi", reqwest::multipart::Part::from(val));
        }
        if let Some(val) = &request.data.version {
            form_data = form_data
                .part("version", reqwest::multipart::Part::text(val.to_string()));
        }
        builder = builder.multipart(form_data);
        builder = self
            .base_client
            .apply_auths_to_builder(builder, &["ApiKeyAuth", "CookieAuth"]);
        let mut response = builder.send().await?;
        response = self.base_client.error_for_status(response).await?;
        crate::core::response::process_json::<crate::models::ApiSpec>(response).await
    }
    /// no description available
    pub async fn create(
        &self,
        request: super::request_types::CreateRequest,
    ) -> crate::SdkResult<crate::models::ApiSpec> {
        let url = self
            .base_client
            .build_url(&format!("/api/{}/spec", & request.api_name));
        let mut builder = reqwest::Client::default().post(&url);
        builder = builder.header("x-sideko-sdk-language", "rust");
        let mut form_data = reqwest::multipart::Form::new();
        if let Some(val) = &request.data.mock_server_enabled {
            form_data = form_data
                .part(
                    "mock_server_enabled",
                    reqwest::multipart::Part::text(val.to_string()),
                );
        }
        if let Some(val) = &request.data.notes {
            form_data = form_data
                .part("notes", reqwest::multipart::Part::text(val.to_string()));
        }
        form_data = form_data
            .part("openapi", reqwest::multipart::Part::from(&request.data.openapi));
        form_data = form_data
            .part(
                "version",
                reqwest::multipart::Part::text(request.data.version.to_string()),
            );
        builder = builder.multipart(form_data);
        builder = self
            .base_client
            .apply_auths_to_builder(builder, &["ApiKeyAuth", "CookieAuth"]);
        let mut response = builder.send().await?;
        response = self.base_client.error_for_status(response).await?;
        crate::core::response::process_json::<crate::models::ApiSpec>(response).await
    }
}
