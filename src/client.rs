#[derive(Clone, Debug, Default)]
pub struct Client {
    base_client: crate::core::base_client::BaseClient,
}
impl Client {
    /// Override the default URL environment
    pub fn with_environment(mut self, env: crate::environment::Environment) -> Self {
        self.base_client.base_url = crate::environment::BaseUrl::Env(env);
        self
    }
    /// Override the default URL with a custom base URL
    pub fn with_base_url(mut self, base_url: &str) -> Self {
        self.base_client.base_url = crate::environment::BaseUrl::Custom(base_url.into());
        self
    }
    /// Override the default underlying reqwest client
    pub fn with_client(mut self, client: reqwest::Client) -> Self {
        self.base_client.client = client;
        self
    }
    pub fn with_api_key_auth(mut self, val: &str) -> Self {
        self.base_client
            .auth
            .insert(
                "ApiKeyAuth".into(),
                crate::core::auth::AuthProvider::KeyHeader(
                    "x-sideko-key".into(),
                    val.into(),
                ),
            );
        self
    }
    pub fn with_cookie_auth(mut self, val: &str) -> Self {
        self.base_client
            .auth
            .insert(
                "CookieAuth".into(),
                crate::core::auth::AuthProvider::KeyCookie(
                    "SIDEKO_SESSION".into(),
                    val.into(),
                ),
            );
        self
    }
    pub fn api(&self) -> crate::resources::api::resource_client::ApiClient {
        crate::resources::api::resource_client::ApiClient::new(self.base_client.clone())
    }
    pub fn role(&self) -> crate::resources::role::resource_client::RoleClient {
        crate::resources::role::resource_client::RoleClient::new(
            self.base_client.clone(),
        )
    }
    pub fn sdk(&self) -> crate::resources::sdk::resource_client::SdkClient {
        crate::resources::sdk::resource_client::SdkClient::new(self.base_client.clone())
    }
    /// no description available
    pub async fn delete_api_link(
        &self,
        request: super::request_types::DeleteApiLinkRequest,
    ) -> crate::SdkResult<()> {
        let url = self
            .base_client
            .build_url(
                &format!(
                    "/api_link/{}", crate ::core::params::format_string_param(& request
                    .id)
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
    pub async fn delete_api_link_group(
        &self,
        request: super::request_types::DeleteApiLinkGroupRequest,
    ) -> crate::SdkResult<()> {
        let url = self
            .base_client
            .build_url(
                &format!(
                    "/api_link_group/{}", crate ::core::params::format_string_param(&
                    request.id)
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
    pub async fn delete_doc_project(
        &self,
        request: super::request_types::DeleteDocProjectRequest,
    ) -> crate::SdkResult<()> {
        let url = self
            .base_client
            .build_url(
                &format!(
                    "/doc_project/{}", crate ::core::params::format_string_param(&
                    request.doc_name)
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
    pub async fn delete_guide(
        &self,
        request: super::request_types::DeleteGuideRequest,
    ) -> crate::SdkResult<()> {
        let url = self
            .base_client
            .build_url(
                &format!(
                    "/doc_project/{}/version/{}/guide/{}", crate
                    ::core::params::format_string_param(& request.doc_name), crate
                    ::core::params::format_string_param(& request.doc_version), crate
                    ::core::params::format_string_param(& request.guide_id)
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
    pub async fn delete_guide_href(
        &self,
        request: super::request_types::DeleteGuideHrefRequest,
    ) -> crate::SdkResult<()> {
        let url = self
            .base_client
            .build_url(
                &format!(
                    "/doc_project/{}/version/{}/guide/{}/href", crate
                    ::core::params::format_string_param(& request.doc_name), crate
                    ::core::params::format_string_param(& request.doc_version), crate
                    ::core::params::format_string_param(& request.guide_id)
                ),
            );
        let mut builder = reqwest::Client::default().delete(&url);
        let mut queries = crate::core::params::QueryParams::default();
        queries.add("variant", &request.variant, false);
        builder = builder.query(&queries.params);
        builder = builder.header("x-sideko-sdk-language", "rust");
        builder = self
            .base_client
            .apply_auths_to_builder(builder, &["ApiKeyAuth", "CookieAuth"]);
        let response = builder.send().await?;
        self.base_client.error_for_status(response).await?;
        Ok(())
    }
    /// Delete a media asset in your organization
    pub async fn delete_asset(
        &self,
        request: super::request_types::DeleteAssetRequest,
    ) -> crate::SdkResult<()> {
        let url = self
            .base_client
            .build_url(
                &format!(
                    "/organization/asset/{}", crate ::core::params::format_string_param(&
                    request.id)
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
    pub async fn delete_service_account(
        &self,
        request: super::request_types::DeleteServiceAccountRequest,
    ) -> crate::SdkResult<()> {
        let url = self
            .base_client
            .build_url(
                &format!(
                    "/user/service_account/{}", crate
                    ::core::params::format_string_param(& request.id)
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
    pub async fn list_api_links(
        &self,
        request: super::request_types::ListApiLinksRequest,
    ) -> crate::SdkResult<Vec<crate::models::ApiLink>> {
        let url = self.base_client.build_url("/api_link");
        let mut builder = reqwest::Client::default().get(&url);
        let mut queries = crate::core::params::QueryParams::default();
        queries.add("doc_version", &request.doc_version, false);
        builder = builder.query(&queries.params);
        builder = builder.header("x-sideko-sdk-language", "rust");
        builder = self
            .base_client
            .apply_auths_to_builder(builder, &["ApiKeyAuth", "CookieAuth"]);
        let mut response = builder.send().await?;
        response = self.base_client.error_for_status(response).await?;
        crate::core::response::process_json::<Vec<crate::models::ApiLink>>(response)
            .await
    }
    /// no description available
    pub async fn get_api_link(
        &self,
        request: super::request_types::GetApiLinkRequest,
    ) -> crate::SdkResult<crate::models::ApiLink> {
        let url = self
            .base_client
            .build_url(
                &format!(
                    "/api_link/{}", crate ::core::params::format_string_param(& request
                    .id)
                ),
            );
        let mut builder = reqwest::Client::default().get(&url);
        builder = builder.header("x-sideko-sdk-language", "rust");
        builder = self
            .base_client
            .apply_auths_to_builder(builder, &["ApiKeyAuth", "CookieAuth"]);
        let mut response = builder.send().await?;
        response = self.base_client.error_for_status(response).await?;
        crate::core::response::process_json::<crate::models::ApiLink>(response).await
    }
    /// no description available
    pub async fn list_api_link_groups(
        &self,
        request: super::request_types::ListApiLinkGroupsRequest,
    ) -> crate::SdkResult<Vec<crate::models::ApiLinkGroup>> {
        let url = self.base_client.build_url("/api_link_group");
        let mut builder = reqwest::Client::default().get(&url);
        let mut queries = crate::core::params::QueryParams::default();
        queries.add("doc_version", &request.doc_version, false);
        builder = builder.query(&queries.params);
        builder = builder.header("x-sideko-sdk-language", "rust");
        builder = self
            .base_client
            .apply_auths_to_builder(builder, &["ApiKeyAuth", "CookieAuth"]);
        let mut response = builder.send().await?;
        response = self.base_client.error_for_status(response).await?;
        crate::core::response::process_json::<Vec<crate::models::ApiLinkGroup>>(response)
            .await
    }
    /// no description available
    pub async fn exchange_code_for_key(
        &self,
        request: super::request_types::ExchangeCodeForKeyRequest,
    ) -> crate::SdkResult<crate::models::UserApiKey> {
        let url = self.base_client.build_url("/auth/exchange_key");
        let mut builder = reqwest::Client::default().get(&url);
        let mut queries = crate::core::params::QueryParams::default();
        queries.add("code", &request.code, false);
        builder = builder.query(&queries.params);
        builder = builder.header("x-sideko-sdk-language", "rust");
        let mut response = builder.send().await?;
        response = self.base_client.error_for_status(response).await?;
        crate::core::response::process_json::<crate::models::UserApiKey>(response).await
    }
    /// no description available
    pub async fn login_callback(
        &self,
        request: super::request_types::LoginCallbackRequest,
    ) -> crate::SdkResult<()> {
        let url = self.base_client.build_url("/auth/login_callback");
        let mut builder = reqwest::Client::default().get(&url);
        let mut queries = crate::core::params::QueryParams::default();
        queries.add("code", &request.code, false);
        queries.add_option("state", &request.state, false);
        builder = builder.query(&queries.params);
        builder = builder.header("x-sideko-sdk-language", "rust");
        let response = builder.send().await?;
        self.base_client.error_for_status(response).await?;
        Ok(())
    }
    /// no description available
    pub async fn login_url(
        &self,
        request: super::request_types::LoginUrlRequest,
    ) -> crate::SdkResult<()> {
        let url = self.base_client.build_url("/auth/login_url");
        let mut builder = reqwest::Client::default().get(&url);
        let mut queries = crate::core::params::QueryParams::default();
        queries.add_option("cli_output", &request.cli_output, false);
        queries.add_option("cli_port", &request.cli_port, false);
        builder = builder.query(&queries.params);
        builder = builder.header("x-sideko-sdk-language", "rust");
        let response = builder.send().await?;
        self.base_client.error_for_status(response).await?;
        Ok(())
    }
    /// no description available
    pub async fn cli_check_updates(
        &self,
        request: super::request_types::CliCheckUpdatesRequest,
    ) -> crate::SdkResult<Vec<crate::models::CliUpdate>> {
        let url = self
            .base_client
            .build_url(
                &format!(
                    "/cli/updates/{}", crate ::core::params::format_string_param(&
                    request.cli_version)
                ),
            );
        let mut builder = reqwest::Client::default().get(&url);
        builder = builder.header("x-sideko-sdk-language", "rust");
        let mut response = builder.send().await?;
        response = self.base_client.error_for_status(response).await?;
        crate::core::response::process_json::<Vec<crate::models::CliUpdate>>(response)
            .await
    }
    /// no description available
    pub async fn list_doc_projects(
        &self,
    ) -> crate::SdkResult<Vec<crate::models::DocProject>> {
        let url = self.base_client.build_url("/doc_project");
        let mut builder = reqwest::Client::default().get(&url);
        builder = builder.header("x-sideko-sdk-language", "rust");
        builder = self
            .base_client
            .apply_auths_to_builder(builder, &["ApiKeyAuth", "CookieAuth"]);
        let mut response = builder.send().await?;
        response = self.base_client.error_for_status(response).await?;
        crate::core::response::process_json::<Vec<crate::models::DocProject>>(response)
            .await
    }
    /// no description available
    pub async fn get_doc_project(
        &self,
        request: super::request_types::GetDocProjectRequest,
    ) -> crate::SdkResult<crate::models::DocProject> {
        let url = self
            .base_client
            .build_url(
                &format!(
                    "/doc_project/{}", crate ::core::params::format_string_param(&
                    request.doc_name)
                ),
            );
        let mut builder = reqwest::Client::default().get(&url);
        builder = builder.header("x-sideko-sdk-language", "rust");
        builder = self
            .base_client
            .apply_auths_to_builder(builder, &["ApiKeyAuth", "CookieAuth"]);
        let mut response = builder.send().await?;
        response = self.base_client.error_for_status(response).await?;
        crate::core::response::process_json::<crate::models::DocProject>(response).await
    }
    /// Retrieves all deployments for a doc project
    pub async fn list_deployments(
        &self,
        request: super::request_types::ListDeploymentsRequest,
    ) -> crate::SdkResult<Vec<crate::models::Deployment>> {
        let url = self
            .base_client
            .build_url(
                &format!(
                    "/doc_project/{}/deployment", crate
                    ::core::params::format_string_param(& request.doc_name)
                ),
            );
        let mut builder = reqwest::Client::default().get(&url);
        let mut queries = crate::core::params::QueryParams::default();
        queries.add_option("limit", &request.limit, false);
        queries.add_option("target", &request.target, false);
        builder = builder.query(&queries.params);
        builder = builder.header("x-sideko-sdk-language", "rust");
        builder = self
            .base_client
            .apply_auths_to_builder(builder, &["ApiKeyAuth", "CookieAuth"]);
        let mut response = builder.send().await?;
        response = self.base_client.error_for_status(response).await?;
        crate::core::response::process_json::<Vec<crate::models::Deployment>>(response)
            .await
    }
    /// Retrieves single deployment
    pub async fn get_deployment(
        &self,
        request: super::request_types::GetDeploymentRequest,
    ) -> crate::SdkResult<crate::models::Deployment> {
        let url = self
            .base_client
            .build_url(
                &format!(
                    "/doc_project/{}/deployment/{}", crate
                    ::core::params::format_string_param(& request.doc_name), crate
                    ::core::params::format_string_param(& request.deployment_id)
                ),
            );
        let mut builder = reqwest::Client::default().get(&url);
        builder = builder.header("x-sideko-sdk-language", "rust");
        builder = self
            .base_client
            .apply_auths_to_builder(builder, &["ApiKeyAuth", "CookieAuth"]);
        let mut response = builder.send().await?;
        response = self.base_client.error_for_status(response).await?;
        crate::core::response::process_json::<crate::models::Deployment>(response).await
    }
    /// no description available
    pub async fn get_doc_project_theme(
        &self,
        request: super::request_types::GetDocProjectThemeRequest,
    ) -> crate::SdkResult<crate::models::Theme> {
        let url = self
            .base_client
            .build_url(
                &format!(
                    "/doc_project/{}/theme", crate ::core::params::format_string_param(&
                    request.doc_name)
                ),
            );
        let mut builder = reqwest::Client::default().get(&url);
        builder = builder.header("x-sideko-sdk-language", "rust");
        builder = self
            .base_client
            .apply_auths_to_builder(builder, &["ApiKeyAuth", "CookieAuth"]);
        let mut response = builder.send().await?;
        response = self.base_client.error_for_status(response).await?;
        crate::core::response::process_json::<crate::models::Theme>(response).await
    }
    /// no description available
    pub async fn list_doc_versions(
        &self,
        request: super::request_types::ListDocVersionsRequest,
    ) -> crate::SdkResult<Vec<crate::models::DocVersion>> {
        let url = self
            .base_client
            .build_url(
                &format!(
                    "/doc_project/{}/version", crate
                    ::core::params::format_string_param(& request.doc_name)
                ),
            );
        let mut builder = reqwest::Client::default().get(&url);
        builder = builder.header("x-sideko-sdk-language", "rust");
        builder = self
            .base_client
            .apply_auths_to_builder(builder, &["ApiKeyAuth", "CookieAuth"]);
        let mut response = builder.send().await?;
        response = self.base_client.error_for_status(response).await?;
        crate::core::response::process_json::<Vec<crate::models::DocVersion>>(response)
            .await
    }
    /// no description available
    pub async fn get_doc_version(
        &self,
        request: super::request_types::GetDocVersionRequest,
    ) -> crate::SdkResult<crate::models::DocVersion> {
        let url = self
            .base_client
            .build_url(
                &format!(
                    "/doc_project/{}/version/{}", crate
                    ::core::params::format_string_param(& request.doc_name), crate
                    ::core::params::format_string_param(& request.doc_version)
                ),
            );
        let mut builder = reqwest::Client::default().get(&url);
        builder = builder.header("x-sideko-sdk-language", "rust");
        builder = self
            .base_client
            .apply_auths_to_builder(builder, &["ApiKeyAuth", "CookieAuth"]);
        let mut response = builder.send().await?;
        response = self.base_client.error_for_status(response).await?;
        crate::core::response::process_json::<crate::models::DocVersion>(response).await
    }
    /// no description available
    pub async fn list_guides(
        &self,
        request: super::request_types::ListGuidesRequest,
    ) -> crate::SdkResult<Vec<crate::models::GuideWithChildren>> {
        let url = self
            .base_client
            .build_url(
                &format!(
                    "/doc_project/{}/version/{}/guide", crate
                    ::core::params::format_string_param(& request.doc_name), crate
                    ::core::params::format_string_param(& request.doc_version)
                ),
            );
        let mut builder = reqwest::Client::default().get(&url);
        builder = builder.header("x-sideko-sdk-language", "rust");
        builder = self
            .base_client
            .apply_auths_to_builder(builder, &["ApiKeyAuth", "CookieAuth"]);
        let mut response = builder.send().await?;
        response = self.base_client.error_for_status(response).await?;
        crate::core::response::process_json::<
            Vec<crate::models::GuideWithChildren>,
        >(response)
            .await
    }
    /// no description available
    pub async fn get_guide(
        &self,
        request: super::request_types::GetGuideRequest,
    ) -> crate::SdkResult<crate::models::Guide> {
        let url = self
            .base_client
            .build_url(
                &format!(
                    "/doc_project/{}/version/{}/guide/{}", crate
                    ::core::params::format_string_param(& request.doc_name), crate
                    ::core::params::format_string_param(& request.doc_version), crate
                    ::core::params::format_string_param(& request.guide_id)
                ),
            );
        let mut builder = reqwest::Client::default().get(&url);
        builder = builder.header("x-sideko-sdk-language", "rust");
        builder = self
            .base_client
            .apply_auths_to_builder(builder, &["ApiKeyAuth", "CookieAuth"]);
        let mut response = builder.send().await?;
        response = self.base_client.error_for_status(response).await?;
        crate::core::response::process_json::<crate::models::Guide>(response).await
    }
    /// no description available
    pub async fn get_guide_content(
        &self,
        request: super::request_types::GetGuideContentRequest,
    ) -> crate::SdkResult<crate::models::GuideContent> {
        let url = self
            .base_client
            .build_url(
                &format!(
                    "/doc_project/{}/version/{}/guide/{}/content", crate
                    ::core::params::format_string_param(& request.doc_name), crate
                    ::core::params::format_string_param(& request.doc_version), crate
                    ::core::params::format_string_param(& request.guide_id)
                ),
            );
        let mut builder = reqwest::Client::default().get(&url);
        builder = builder.header("x-sideko-sdk-language", "rust");
        builder = self
            .base_client
            .apply_auths_to_builder(builder, &["ApiKeyAuth", "CookieAuth"]);
        let mut response = builder.send().await?;
        response = self.base_client.error_for_status(response).await?;
        crate::core::response::process_json::<crate::models::GuideContent>(response)
            .await
    }
    /// Get user organization
    pub async fn get_organization(
        &self,
    ) -> crate::SdkResult<crate::models::Organization> {
        let url = self.base_client.build_url("/organization");
        let mut builder = reqwest::Client::default().get(&url);
        builder = builder.header("x-sideko-sdk-language", "rust");
        builder = self
            .base_client
            .apply_auths_to_builder(builder, &["ApiKeyAuth", "CookieAuth"]);
        let mut response = builder.send().await?;
        response = self.base_client.error_for_status(response).await?;
        crate::core::response::process_json::<crate::models::Organization>(response)
            .await
    }
    /// Get all media assets for an organization
    pub async fn list_assets(
        &self,
        request: super::request_types::ListAssetsRequest,
    ) -> crate::SdkResult<Vec<crate::models::ListAssetsPage>> {
        let url = self.base_client.build_url("/organization/asset");
        let mut builder = reqwest::Client::default().get(&url);
        let mut queries = crate::core::params::QueryParams::default();
        queries.add_option("name", &request.name, false);
        queries.add_option("page", &request.page, false);
        builder = builder.query(&queries.params);
        builder = builder.header("x-sideko-sdk-language", "rust");
        builder = self
            .base_client
            .apply_auths_to_builder(builder, &["ApiKeyAuth", "CookieAuth"]);
        let mut response = builder.send().await?;
        response = self.base_client.error_for_status(response).await?;
        crate::core::response::process_json::<
            Vec<crate::models::ListAssetsPage>,
        >(response)
            .await
    }
    /// Get documentation project theme configured at the organization level
    pub async fn get_organization_theme(
        &self,
    ) -> crate::SdkResult<crate::models::Theme> {
        let url = self.base_client.build_url("/organization/theme");
        let mut builder = reqwest::Client::default().get(&url);
        builder = builder.header("x-sideko-sdk-language", "rust");
        builder = self
            .base_client
            .apply_auths_to_builder(builder, &["ApiKeyAuth", "CookieAuth"]);
        let mut response = builder.send().await?;
        response = self.base_client.error_for_status(response).await?;
        crate::core::response::process_json::<crate::models::Theme>(response).await
    }
    /// no description available
    pub async fn get_current_user(&self) -> crate::SdkResult<crate::models::User> {
        let url = self.base_client.build_url("/user/me");
        let mut builder = reqwest::Client::default().get(&url);
        builder = builder.header("x-sideko-sdk-language", "rust");
        builder = self
            .base_client
            .apply_auths_to_builder(builder, &["ApiKeyAuth", "CookieAuth"]);
        let mut response = builder.send().await?;
        response = self.base_client.error_for_status(response).await?;
        crate::core::response::process_json::<crate::models::User>(response).await
    }
    /// no description available
    pub async fn get_api_key(&self) -> crate::SdkResult<crate::models::UserApiKey> {
        let url = self.base_client.build_url("/user/me/api_key");
        let mut builder = reqwest::Client::default().get(&url);
        builder = builder.header("x-sideko-sdk-language", "rust");
        builder = self
            .base_client
            .apply_auths_to_builder(builder, &["ApiKeyAuth", "CookieAuth"]);
        let mut response = builder.send().await?;
        response = self.base_client.error_for_status(response).await?;
        crate::core::response::process_json::<crate::models::UserApiKey>(response).await
    }
    /// no description available
    pub async fn list_service_accounts(
        &self,
    ) -> crate::SdkResult<Vec<crate::models::UserApiKey>> {
        let url = self.base_client.build_url("/user/service_account");
        let mut builder = reqwest::Client::default().get(&url);
        builder = builder.header("x-sideko-sdk-language", "rust");
        builder = self
            .base_client
            .apply_auths_to_builder(builder, &["ApiKeyAuth", "CookieAuth"]);
        let mut response = builder.send().await?;
        response = self.base_client.error_for_status(response).await?;
        crate::core::response::process_json::<Vec<crate::models::UserApiKey>>(response)
            .await
    }
    /// no description available
    pub async fn get_service_account(
        &self,
        request: super::request_types::GetServiceAccountRequest,
    ) -> crate::SdkResult<crate::models::UserApiKey> {
        let url = self
            .base_client
            .build_url(
                &format!(
                    "/user/service_account/{}", crate
                    ::core::params::format_string_param(& request.id)
                ),
            );
        let mut builder = reqwest::Client::default().get(&url);
        builder = builder.header("x-sideko-sdk-language", "rust");
        builder = self
            .base_client
            .apply_auths_to_builder(builder, &["ApiKeyAuth", "CookieAuth"]);
        let mut response = builder.send().await?;
        response = self.base_client.error_for_status(response).await?;
        crate::core::response::process_json::<crate::models::UserApiKey>(response).await
    }
    /// no description available
    pub async fn update_api_link(
        &self,
        request: super::request_types::UpdateApiLinkRequest,
    ) -> crate::SdkResult<crate::models::ApiLink> {
        let url = self
            .base_client
            .build_url(
                &format!(
                    "/api_link/{}", crate ::core::params::format_string_param(& request
                    .id)
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
        crate::core::response::process_json::<crate::models::ApiLink>(response).await
    }
    /// no description available
    pub async fn update_api_link_group(
        &self,
        request: super::request_types::UpdateApiLinkGroupRequest,
    ) -> crate::SdkResult<crate::models::ApiLinkGroup> {
        let url = self
            .base_client
            .build_url(
                &format!(
                    "/api_link_group/{}", crate ::core::params::format_string_param(&
                    request.id)
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
        crate::core::response::process_json::<crate::models::ApiLinkGroup>(response)
            .await
    }
    /// no description available
    pub async fn update_doc_project(
        &self,
        request: super::request_types::UpdateDocProjectRequest,
    ) -> crate::SdkResult<crate::models::DocProject> {
        let url = self
            .base_client
            .build_url(
                &format!(
                    "/doc_project/{}", crate ::core::params::format_string_param(&
                    request.doc_name)
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
        crate::core::response::process_json::<crate::models::DocProject>(response).await
    }
    /// no description available
    pub async fn update_guide(
        &self,
        request: super::request_types::UpdateGuideRequest,
    ) -> crate::SdkResult<crate::models::Guide> {
        let url = self
            .base_client
            .build_url(
                &format!(
                    "/doc_project/{}/version/{}/guide/{}", crate
                    ::core::params::format_string_param(& request.doc_name), crate
                    ::core::params::format_string_param(& request.doc_version), crate
                    ::core::params::format_string_param(& request.guide_id)
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
        crate::core::response::process_json::<crate::models::Guide>(response).await
    }
    /// Update a media asset in your organization
    pub async fn update_asset(
        &self,
        request: super::request_types::UpdateAssetRequest,
    ) -> crate::SdkResult<crate::models::Asset> {
        let url = self
            .base_client
            .build_url(
                &format!(
                    "/organization/asset/{}", crate ::core::params::format_string_param(&
                    request.id)
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
        crate::core::response::process_json::<crate::models::Asset>(response).await
    }
    /// no description available
    pub async fn create_api_link(
        &self,
        request: super::request_types::CreateApiLinkRequest,
    ) -> crate::SdkResult<crate::models::ApiLink> {
        let url = self.base_client.build_url("/api_link");
        let mut builder = reqwest::Client::default().post(&url);
        builder = builder.header("x-sideko-sdk-language", "rust");
        builder = builder.header("content-type", "application/json");
        builder = builder.json(&request.data);
        builder = self
            .base_client
            .apply_auths_to_builder(builder, &["ApiKeyAuth", "CookieAuth"]);
        let mut response = builder.send().await?;
        response = self.base_client.error_for_status(response).await?;
        crate::core::response::process_json::<crate::models::ApiLink>(response).await
    }
    /// no description available
    pub async fn reorder_api_links(
        &self,
        request: super::request_types::ReorderApiLinksRequest,
    ) -> crate::SdkResult<crate::models::ApiReorder> {
        let url = self.base_client.build_url("/api_link/reorder");
        let mut builder = reqwest::Client::default().post(&url);
        builder = builder.header("x-sideko-sdk-language", "rust");
        builder = builder.header("content-type", "application/json");
        builder = builder.json(&request.data);
        builder = self
            .base_client
            .apply_auths_to_builder(builder, &["ApiKeyAuth", "CookieAuth"]);
        let mut response = builder.send().await?;
        response = self.base_client.error_for_status(response).await?;
        crate::core::response::process_json::<crate::models::ApiReorder>(response).await
    }
    /// no description available
    pub async fn create_api_link_group(
        &self,
        request: super::request_types::CreateApiLinkGroupRequest,
    ) -> crate::SdkResult<crate::models::ApiLinkGroup> {
        let url = self.base_client.build_url("/api_link_group");
        let mut builder = reqwest::Client::default().post(&url);
        builder = builder.header("x-sideko-sdk-language", "rust");
        builder = builder.header("content-type", "application/json");
        builder = builder.json(&request.data);
        builder = self
            .base_client
            .apply_auths_to_builder(builder, &["ApiKeyAuth", "CookieAuth"]);
        let mut response = builder.send().await?;
        response = self.base_client.error_for_status(response).await?;
        crate::core::response::process_json::<crate::models::ApiLinkGroup>(response)
            .await
    }
    /// no description available
    pub async fn create_doc_project(
        &self,
        request: super::request_types::CreateDocProjectRequest,
    ) -> crate::SdkResult<crate::models::DocProject> {
        let url = self.base_client.build_url("/doc_project");
        let mut builder = reqwest::Client::default().post(&url);
        builder = builder.header("x-sideko-sdk-language", "rust");
        builder = builder.header("content-type", "application/json");
        builder = builder.json(&request.data);
        builder = self
            .base_client
            .apply_auths_to_builder(builder, &["ApiKeyAuth", "CookieAuth"]);
        let mut response = builder.send().await?;
        response = self.base_client.error_for_status(response).await?;
        crate::core::response::process_json::<crate::models::DocProject>(response).await
    }
    /// Deploys a new generated version of documentation with linked guides & APIs
    pub async fn trigger_deployment(
        &self,
        request: super::request_types::TriggerDeploymentRequest,
    ) -> crate::SdkResult<crate::models::Deployment> {
        let url = self
            .base_client
            .build_url(
                &format!(
                    "/doc_project/{}/deployment", crate
                    ::core::params::format_string_param(& request.doc_name)
                ),
            );
        let mut builder = reqwest::Client::default().post(&url);
        builder = builder.header("x-sideko-sdk-language", "rust");
        builder = builder.header("content-type", "application/json");
        builder = builder.json(&request.data);
        builder = self
            .base_client
            .apply_auths_to_builder(builder, &["ApiKeyAuth", "CookieAuth"]);
        let mut response = builder.send().await?;
        response = self.base_client.error_for_status(response).await?;
        crate::core::response::process_json::<crate::models::Deployment>(response).await
    }
    /// no description available
    pub async fn create_guide(
        &self,
        request: super::request_types::CreateGuideRequest,
    ) -> crate::SdkResult<crate::models::Guide> {
        let url = self
            .base_client
            .build_url(
                &format!(
                    "/doc_project/{}/version/{}/guide", crate
                    ::core::params::format_string_param(& request.doc_name), crate
                    ::core::params::format_string_param(& request.doc_version)
                ),
            );
        let mut builder = reqwest::Client::default().post(&url);
        builder = builder.header("x-sideko-sdk-language", "rust");
        builder = builder.header("content-type", "application/json");
        builder = builder.json(&request.data);
        builder = self
            .base_client
            .apply_auths_to_builder(builder, &["ApiKeyAuth", "CookieAuth"]);
        let mut response = builder.send().await?;
        response = self.base_client.error_for_status(response).await?;
        crate::core::response::process_json::<crate::models::Guide>(response).await
    }
    /// no description available
    pub async fn reorder_guides(
        &self,
        request: super::request_types::ReorderGuidesRequest,
    ) -> crate::SdkResult<Vec<crate::models::GuideWithChildren>> {
        let url = self
            .base_client
            .build_url(
                &format!(
                    "/doc_project/{}/version/{}/guide/reorder", crate
                    ::core::params::format_string_param(& request.doc_name), crate
                    ::core::params::format_string_param(& request.doc_version)
                ),
            );
        let mut builder = reqwest::Client::default().post(&url);
        builder = builder.header("x-sideko-sdk-language", "rust");
        builder = builder.header("content-type", "application/json");
        builder = builder.json(&request.data);
        builder = self
            .base_client
            .apply_auths_to_builder(builder, &["ApiKeyAuth", "CookieAuth"]);
        let mut response = builder.send().await?;
        response = self.base_client.error_for_status(response).await?;
        crate::core::response::process_json::<
            Vec<crate::models::GuideWithChildren>,
        >(response)
            .await
    }
    /// no description available
    pub async fn create_organization(
        &self,
        request: super::request_types::CreateOrganizationRequest,
    ) -> crate::SdkResult<crate::models::OrganizationWithRedirect> {
        let url = self.base_client.build_url("/organization");
        let mut builder = reqwest::Client::default().post(&url);
        builder = builder.header("x-sideko-sdk-language", "rust");
        builder = builder.header("content-type", "application/json");
        builder = builder.json(&request.data);
        builder = self
            .base_client
            .apply_auths_to_builder(builder, &["ApiKeyAuth", "CookieAuth"]);
        let mut response = builder.send().await?;
        response = self.base_client.error_for_status(response).await?;
        crate::core::response::process_json::<
            crate::models::OrganizationWithRedirect,
        >(response)
            .await
    }
    /// Add a media asset like logos or other media to an organization
    pub async fn upload_assets(
        &self,
        request: super::request_types::UploadAssetsRequest,
    ) -> crate::SdkResult<Vec<crate::models::Asset>> {
        let url = self.base_client.build_url("/organization/asset");
        let mut builder = reqwest::Client::default().post(&url);
        builder = builder.header("x-sideko-sdk-language", "rust");
        let mut form_data = reqwest::multipart::Form::new();
        form_data = form_data
            .part("file", reqwest::multipart::Part::from(&request.data.file));
        builder = builder.multipart(form_data);
        builder = self
            .base_client
            .apply_auths_to_builder(builder, &["ApiKeyAuth", "CookieAuth"]);
        let mut response = builder.send().await?;
        response = self.base_client.error_for_status(response).await?;
        crate::core::response::process_json::<Vec<crate::models::Asset>>(response).await
    }
    /// no description available
    pub async fn invite_user(
        &self,
        request: super::request_types::InviteUserRequest,
    ) -> crate::SdkResult<()> {
        let url = self.base_client.build_url("/user/invite");
        let mut builder = reqwest::Client::default().post(&url);
        builder = builder.header("x-sideko-sdk-language", "rust");
        builder = builder.header("content-type", "application/json");
        builder = builder.json(&request.data);
        builder = self
            .base_client
            .apply_auths_to_builder(builder, &["ApiKeyAuth", "CookieAuth"]);
        let response = builder.send().await?;
        self.base_client.error_for_status(response).await?;
        Ok(())
    }
    /// no description available
    pub async fn create_service_account(
        &self,
        request: super::request_types::CreateServiceAccountRequest,
    ) -> crate::SdkResult<crate::models::UserApiKey> {
        let url = self.base_client.build_url("/user/service_account");
        let mut builder = reqwest::Client::default().post(&url);
        builder = builder.header("x-sideko-sdk-language", "rust");
        builder = builder.header("content-type", "application/json");
        builder = builder.json(&request.data);
        builder = self
            .base_client
            .apply_auths_to_builder(builder, &["ApiKeyAuth", "CookieAuth"]);
        let mut response = builder.send().await?;
        response = self.base_client.error_for_status(response).await?;
        crate::core::response::process_json::<crate::models::UserApiKey>(response).await
    }
    /// no description available
    pub async fn update_doc_project_theme(
        &self,
        request: super::request_types::UpdateDocProjectThemeRequest,
    ) -> crate::SdkResult<crate::models::Theme> {
        let url = self
            .base_client
            .build_url(
                &format!(
                    "/doc_project/{}/theme", crate ::core::params::format_string_param(&
                    request.doc_name)
                ),
            );
        let mut builder = reqwest::Client::default().put(&url);
        builder = builder.header("x-sideko-sdk-language", "rust");
        builder = builder.header("content-type", "application/json");
        builder = builder.json(&request.data);
        builder = self
            .base_client
            .apply_auths_to_builder(builder, &["ApiKeyAuth", "CookieAuth"]);
        let mut response = builder.send().await?;
        response = self.base_client.error_for_status(response).await?;
        crate::core::response::process_json::<crate::models::Theme>(response).await
    }
    /// Update  documentation project theme configured at the organization level
    pub async fn update_organization_theme(
        &self,
        request: super::request_types::UpdateOrganizationThemeRequest,
    ) -> crate::SdkResult<crate::models::Theme> {
        let url = self.base_client.build_url("/organization/theme");
        let mut builder = reqwest::Client::default().put(&url);
        builder = builder.header("x-sideko-sdk-language", "rust");
        builder = builder.header("content-type", "application/json");
        builder = builder.json(&request.data);
        builder = self
            .base_client
            .apply_auths_to_builder(builder, &["ApiKeyAuth", "CookieAuth"]);
        let mut response = builder.send().await?;
        response = self.base_client.error_for_status(response).await?;
        crate::core::response::process_json::<crate::models::Theme>(response).await
    }
}
