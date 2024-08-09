/// Generatedby Sideko (sideko.dev)
use crate::auth;
use crate::request_types::*;
use crate::result;
use crate::error_enums;
use crate::schemas::*;
use reqwest::blocking::Client as ReqwestClient;
use reqwest::blocking::RequestBuilder as ReqwestRequestBuilder;
#[allow(unused)]
use reqwest::blocking::multipart as reqwest_multipart;
use std::collections::BTreeMap;
#[derive(Clone, Debug)]
pub struct Client {
    pub base_url: String,
    auth: BTreeMap<String, auth::AuthProvider>,
}
impl Default for Client {
    fn default() -> Self {
        Self {
            base_url: "https://api.sideko.dev/v1".to_string(),
            auth: BTreeMap::new(),
        }
    }
}
impl Client {
    /// Override the default base url
    pub fn with_base_url(mut self, base_url: &str) -> Self {
        self.base_url = base_url.into();
        self
    }
    /// Authentication  builder function to store api-key credentials in the client
    pub fn with_api_key_auth(mut self, val: &str) -> Self {
        self.auth
            .insert(
                "ApiKeyAuth".to_string(),
                auth::AuthProvider::KeyHeader(
                    "x-sideko-key".to_string(),
                    val.to_string(),
                ),
            );
        self
    }
    /// Authentication  builder function to store api-key credentials in the client
    pub fn with_cookie_auth(mut self, val: &str) -> Self {
        self.auth
            .insert(
                "CookieAuth".to_string(),
                auth::AuthProvider::KeyCookie("sideko_jwt".to_string(), val.to_string()),
            );
        self
    }
    fn builder_with_auth(
        &self,
        mut req_builder: ReqwestRequestBuilder,
        auth_names: &[&str],
    ) -> ReqwestRequestBuilder {
        for auth_name in auth_names {
            if let Some(provider) = self.auth.get(&auth_name.to_string()) {
                req_builder = provider.add_auth_blocking(req_builder);
            }
        }
        req_builder
    }
    pub fn delete_api_link(
        &self,
        request: DeleteApiLinkRequest,
    ) -> result::Result<serde_json::Value, error_enums::DeleteApiLinkErrors> {
        let endpoint = format!("/api_link/{}", request.link_id);
        let url = format!("{}{}", self.base_url, endpoint);
        let query_params: Vec<(&str, String)> = vec![];
        let unauthed_builder = ReqwestClient::default()
            .delete(&url)
            .query(&query_params);
        let authed_builder = self
            .builder_with_auth(unauthed_builder, &["ApiKeyAuth", "CookieAuth"]);
        let response = authed_builder.send().map_err(result::Error::Dispatch)?;
        let status_code = response.status().as_u16();
        match status_code {
            204 => {
                let response_text = response.text().unwrap_or_default();
                if response_text.is_empty() {
                    return Ok(serde_json::json!({}));
                }
                let data = serde_json::from_str::<serde_json::Value>(&response_text)
                    .unwrap();
                Ok(data)
            }
            _ => {
                Err(result::Error::BlockingApiError {
                    status_code,
                    method: "".to_string(),
                    url: url.to_string(),
                    response,
                })
            }
        }
    }
    pub fn delete_api_link_group(
        &self,
        request: DeleteApiLinkGroupRequest,
    ) -> result::Result<serde_json::Value, error_enums::DeleteApiLinkGroupErrors> {
        let endpoint = format!("/api_link_group/{}", request.group_id);
        let url = format!("{}{}", self.base_url, endpoint);
        let query_params: Vec<(&str, String)> = vec![];
        let unauthed_builder = ReqwestClient::default()
            .delete(&url)
            .query(&query_params);
        let authed_builder = self
            .builder_with_auth(unauthed_builder, &["ApiKeyAuth", "CookieAuth"]);
        let response = authed_builder.send().map_err(result::Error::Dispatch)?;
        let status_code = response.status().as_u16();
        match status_code {
            204 => {
                let response_text = response.text().unwrap_or_default();
                if response_text.is_empty() {
                    return Ok(serde_json::json!({}));
                }
                let data = serde_json::from_str::<serde_json::Value>(&response_text)
                    .unwrap();
                Ok(data)
            }
            _ => {
                Err(result::Error::BlockingApiError {
                    status_code,
                    method: "".to_string(),
                    url: url.to_string(),
                    response,
                })
            }
        }
    }
    pub fn delete_api_project(
        &self,
        request: DeleteApiProjectRequest,
    ) -> result::Result<serde_json::Value, error_enums::DeleteApiProjectErrors> {
        let endpoint = format!("/api_project/{}", request.project_id_or_name);
        let url = format!("{}{}", self.base_url, endpoint);
        let query_params: Vec<(&str, String)> = vec![];
        let unauthed_builder = ReqwestClient::default()
            .delete(&url)
            .query(&query_params);
        let authed_builder = self
            .builder_with_auth(unauthed_builder, &["ApiKeyAuth", "CookieAuth"]);
        let response = authed_builder.send().map_err(result::Error::Dispatch)?;
        let status_code = response.status().as_u16();
        match status_code {
            204 => {
                let response_text = response.text().unwrap_or_default();
                if response_text.is_empty() {
                    return Ok(serde_json::json!({}));
                }
                let data = serde_json::from_str::<serde_json::Value>(&response_text)
                    .unwrap();
                Ok(data)
            }
            _ => {
                Err(result::Error::BlockingApiError {
                    status_code,
                    method: "".to_string(),
                    url: url.to_string(),
                    response,
                })
            }
        }
    }
    pub fn delete_api_project_role(
        &self,
        request: DeleteApiProjectRoleRequest,
    ) -> result::Result<serde_json::Value, error_enums::DeleteApiProjectRoleErrors> {
        let endpoint = format!(
            "/api_project/{}/role/{}", request.project_id_or_name, request.user_id
        );
        let url = format!("{}{}", self.base_url, endpoint);
        let query_params: Vec<(&str, String)> = vec![];
        let unauthed_builder = ReqwestClient::default()
            .delete(&url)
            .query(&query_params);
        let authed_builder = self
            .builder_with_auth(unauthed_builder, &["ApiKeyAuth", "CookieAuth"]);
        let response = authed_builder.send().map_err(result::Error::Dispatch)?;
        let status_code = response.status().as_u16();
        match status_code {
            204 => {
                let response_text = response.text().unwrap_or_default();
                if response_text.is_empty() {
                    return Ok(serde_json::json!({}));
                }
                let data = serde_json::from_str::<serde_json::Value>(&response_text)
                    .unwrap();
                Ok(data)
            }
            _ => {
                Err(result::Error::BlockingApiError {
                    status_code,
                    method: "".to_string(),
                    url: url.to_string(),
                    response,
                })
            }
        }
    }
    pub fn delete_doc_project(
        &self,
        request: DeleteDocProjectRequest,
    ) -> result::Result<serde_json::Value, error_enums::DeleteDocProjectErrors> {
        let endpoint = format!("/doc_project/{}", request.project_id_or_name);
        let url = format!("{}{}", self.base_url, endpoint);
        let query_params: Vec<(&str, String)> = vec![];
        let unauthed_builder = ReqwestClient::default()
            .delete(&url)
            .query(&query_params);
        let authed_builder = self
            .builder_with_auth(unauthed_builder, &["ApiKeyAuth", "CookieAuth"]);
        let response = authed_builder.send().map_err(result::Error::Dispatch)?;
        let status_code = response.status().as_u16();
        match status_code {
            204 => {
                let response_text = response.text().unwrap_or_default();
                if response_text.is_empty() {
                    return Ok(serde_json::json!({}));
                }
                let data = serde_json::from_str::<serde_json::Value>(&response_text)
                    .unwrap();
                Ok(data)
            }
            _ => {
                Err(result::Error::BlockingApiError {
                    status_code,
                    method: "".to_string(),
                    url: url.to_string(),
                    response,
                })
            }
        }
    }
    pub fn delete_doc_project_role(
        &self,
        request: DeleteDocProjectRoleRequest,
    ) -> result::Result<serde_json::Value, error_enums::DeleteDocProjectRoleErrors> {
        let endpoint = format!(
            "/doc_project/{}/role/{}", request.project_id_or_name, request.user_id
        );
        let url = format!("{}{}", self.base_url, endpoint);
        let query_params: Vec<(&str, String)> = vec![];
        let unauthed_builder = ReqwestClient::default()
            .delete(&url)
            .query(&query_params);
        let authed_builder = self
            .builder_with_auth(unauthed_builder, &["ApiKeyAuth", "CookieAuth"]);
        let response = authed_builder.send().map_err(result::Error::Dispatch)?;
        let status_code = response.status().as_u16();
        match status_code {
            204 => {
                let response_text = response.text().unwrap_or_default();
                if response_text.is_empty() {
                    return Ok(serde_json::json!({}));
                }
                let data = serde_json::from_str::<serde_json::Value>(&response_text)
                    .unwrap();
                Ok(data)
            }
            _ => {
                Err(result::Error::BlockingApiError {
                    status_code,
                    method: "".to_string(),
                    url: url.to_string(),
                    response,
                })
            }
        }
    }
    pub fn delete_guide(
        &self,
        request: DeleteGuideRequest,
    ) -> result::Result<serde_json::Value, error_enums::DeleteGuideErrors> {
        let endpoint = format!(
            "/doc_project/{}/version/{}/guide/{}", request.project_id_or_name, request
            .version_id, request.guide_id
        );
        let url = format!("{}{}", self.base_url, endpoint);
        let query_params: Vec<(&str, String)> = vec![];
        let unauthed_builder = ReqwestClient::default()
            .delete(&url)
            .query(&query_params);
        let authed_builder = self
            .builder_with_auth(unauthed_builder, &["ApiKeyAuth", "CookieAuth"]);
        let response = authed_builder.send().map_err(result::Error::Dispatch)?;
        let status_code = response.status().as_u16();
        match status_code {
            204 => {
                let response_text = response.text().unwrap_or_default();
                if response_text.is_empty() {
                    return Ok(serde_json::json!({}));
                }
                let data = serde_json::from_str::<serde_json::Value>(&response_text)
                    .unwrap();
                Ok(data)
            }
            _ => {
                Err(result::Error::BlockingApiError {
                    status_code,
                    method: "".to_string(),
                    url: url.to_string(),
                    response,
                })
            }
        }
    }
    pub fn delete_guide_href(
        &self,
        request: DeleteGuideHrefRequest,
    ) -> result::Result<serde_json::Value, error_enums::DeleteGuideHrefErrors> {
        let endpoint = format!(
            "/doc_project/{}/version/{}/guide/{}/href", request.project_id_or_name,
            request.version_id, request.guide_id
        );
        let url = format!("{}{}", self.base_url, endpoint);
        let mut query_params: Vec<(&str, String)> = vec![];
        query_params.push(("variant", request.variant.to_string()));
        let unauthed_builder = ReqwestClient::default()
            .delete(&url)
            .query(&query_params);
        let authed_builder = self
            .builder_with_auth(unauthed_builder, &["ApiKeyAuth", "CookieAuth"]);
        let response = authed_builder.send().map_err(result::Error::Dispatch)?;
        let status_code = response.status().as_u16();
        match status_code {
            204 => {
                let response_text = response.text().unwrap_or_default();
                if response_text.is_empty() {
                    return Ok(serde_json::json!({}));
                }
                let data = serde_json::from_str::<serde_json::Value>(&response_text)
                    .unwrap();
                Ok(data)
            }
            _ => {
                Err(result::Error::BlockingApiError {
                    status_code,
                    method: "".to_string(),
                    url: url.to_string(),
                    response,
                })
            }
        }
    }
    pub fn delete_service_account(
        &self,
        request: DeleteServiceAccountRequest,
    ) -> result::Result<serde_json::Value, error_enums::DeleteServiceAccountErrors> {
        let endpoint = format!("/user/service_account/{}", request.service_account_id);
        let url = format!("{}{}", self.base_url, endpoint);
        let query_params: Vec<(&str, String)> = vec![];
        let unauthed_builder = ReqwestClient::default()
            .delete(&url)
            .query(&query_params);
        let authed_builder = self
            .builder_with_auth(unauthed_builder, &["ApiKeyAuth", "CookieAuth"]);
        let response = authed_builder.send().map_err(result::Error::Dispatch)?;
        let status_code = response.status().as_u16();
        match status_code {
            204 => {
                let response_text = response.text().unwrap_or_default();
                if response_text.is_empty() {
                    return Ok(serde_json::json!({}));
                }
                let data = serde_json::from_str::<serde_json::Value>(&response_text)
                    .unwrap();
                Ok(data)
            }
            _ => {
                Err(result::Error::BlockingApiError {
                    status_code,
                    method: "".to_string(),
                    url: url.to_string(),
                    response,
                })
            }
        }
    }
    pub fn health_check(
        &self,
    ) -> result::Result<GetHealthResponse, error_enums::HealthCheckErrors> {
        let endpoint = "/_health";
        let url = format!("{}{}", self.base_url, endpoint);
        let query_params: Vec<(&str, String)> = vec![];
        let unauthed_builder = ReqwestClient::default().get(&url).query(&query_params);
        let authed_builder = self
            .builder_with_auth(unauthed_builder, &["ApiKeyAuth", "CookieAuth"]);
        let response = authed_builder.send().map_err(result::Error::Dispatch)?;
        let status_code = response.status().as_u16();
        match status_code {
            200 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<GetHealthResponse>(&response_text)
                    .unwrap();
                Ok(data)
            }
            _ => {
                Err(result::Error::BlockingApiError {
                    status_code,
                    method: "".to_string(),
                    url: url.to_string(),
                    response,
                })
            }
        }
    }
    pub fn ping_check(
        &self,
    ) -> result::Result<GetPingResponse, error_enums::PingCheckErrors> {
        let endpoint = "/_ping";
        let url = format!("{}{}", self.base_url, endpoint);
        let query_params: Vec<(&str, String)> = vec![];
        let unauthed_builder = ReqwestClient::default().get(&url).query(&query_params);
        let authed_builder = self.builder_with_auth(unauthed_builder, &[]);
        let response = authed_builder.send().map_err(result::Error::Dispatch)?;
        let status_code = response.status().as_u16();
        match status_code {
            200 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<GetPingResponse>(&response_text)
                    .unwrap();
                Ok(data)
            }
            _ => {
                Err(result::Error::BlockingApiError {
                    status_code,
                    method: "".to_string(),
                    url: url.to_string(),
                    response,
                })
            }
        }
    }
    pub fn list_api_links(
        &self,
        request: ListApiLinksRequest,
    ) -> result::Result<Vec<ApiLink>, error_enums::ListApiLinksErrors> {
        let endpoint = "/api_link";
        let url = format!("{}{}", self.base_url, endpoint);
        let mut query_params: Vec<(&str, String)> = vec![];
        query_params.push(("doc_version_id", request.doc_version_id.to_string()));
        let unauthed_builder = ReqwestClient::default().get(&url).query(&query_params);
        let authed_builder = self
            .builder_with_auth(unauthed_builder, &["ApiKeyAuth", "CookieAuth"]);
        let response = authed_builder.send().map_err(result::Error::Dispatch)?;
        let status_code = response.status().as_u16();
        match status_code {
            200 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Vec<ApiLink>>(&response_text).unwrap();
                Ok(data)
            }
            _ => {
                Err(result::Error::BlockingApiError {
                    status_code,
                    method: "".to_string(),
                    url: url.to_string(),
                    response,
                })
            }
        }
    }
    pub fn get_api_link(
        &self,
        request: GetApiLinkRequest,
    ) -> result::Result<ApiLink, error_enums::GetApiLinkErrors> {
        let endpoint = format!("/api_link/{}", request.link_id);
        let url = format!("{}{}", self.base_url, endpoint);
        let query_params: Vec<(&str, String)> = vec![];
        let unauthed_builder = ReqwestClient::default().get(&url).query(&query_params);
        let authed_builder = self
            .builder_with_auth(unauthed_builder, &["ApiKeyAuth", "CookieAuth"]);
        let response = authed_builder.send().map_err(result::Error::Dispatch)?;
        let status_code = response.status().as_u16();
        match status_code {
            200 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<ApiLink>(&response_text).unwrap();
                Ok(data)
            }
            _ => {
                Err(result::Error::BlockingApiError {
                    status_code,
                    method: "".to_string(),
                    url: url.to_string(),
                    response,
                })
            }
        }
    }
    pub fn list_api_link_groups(
        &self,
        request: ListApiLinkGroupsRequest,
    ) -> result::Result<Vec<ApiLinkGroup>, error_enums::ListApiLinkGroupsErrors> {
        let endpoint = "/api_link_group";
        let url = format!("{}{}", self.base_url, endpoint);
        let mut query_params: Vec<(&str, String)> = vec![];
        query_params.push(("doc_version_id", request.doc_version_id.to_string()));
        let unauthed_builder = ReqwestClient::default().get(&url).query(&query_params);
        let authed_builder = self
            .builder_with_auth(unauthed_builder, &["ApiKeyAuth", "CookieAuth"]);
        let response = authed_builder.send().map_err(result::Error::Dispatch)?;
        let status_code = response.status().as_u16();
        match status_code {
            200 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Vec<ApiLinkGroup>>(&response_text)
                    .unwrap();
                Ok(data)
            }
            _ => {
                Err(result::Error::BlockingApiError {
                    status_code,
                    method: "".to_string(),
                    url: url.to_string(),
                    response,
                })
            }
        }
    }
    pub fn list_api_projects(
        &self,
    ) -> result::Result<Vec<ApiProject>, error_enums::ListApiProjectsErrors> {
        let endpoint = "/api_project";
        let url = format!("{}{}", self.base_url, endpoint);
        let query_params: Vec<(&str, String)> = vec![];
        let unauthed_builder = ReqwestClient::default().get(&url).query(&query_params);
        let authed_builder = self
            .builder_with_auth(unauthed_builder, &["ApiKeyAuth", "CookieAuth"]);
        let response = authed_builder.send().map_err(result::Error::Dispatch)?;
        let status_code = response.status().as_u16();
        match status_code {
            200 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Vec<ApiProject>>(&response_text)
                    .unwrap();
                Ok(data)
            }
            _ => {
                Err(result::Error::BlockingApiError {
                    status_code,
                    method: "".to_string(),
                    url: url.to_string(),
                    response,
                })
            }
        }
    }
    pub fn get_api_project(
        &self,
        request: GetApiProjectRequest,
    ) -> result::Result<ApiProject, error_enums::GetApiProjectErrors> {
        let endpoint = format!("/api_project/{}", request.project_id_or_name);
        let url = format!("{}{}", self.base_url, endpoint);
        let query_params: Vec<(&str, String)> = vec![];
        let unauthed_builder = ReqwestClient::default().get(&url).query(&query_params);
        let authed_builder = self
            .builder_with_auth(unauthed_builder, &["ApiKeyAuth", "CookieAuth"]);
        let response = authed_builder.send().map_err(result::Error::Dispatch)?;
        let status_code = response.status().as_u16();
        match status_code {
            200 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<ApiProject>(&response_text).unwrap();
                Ok(data)
            }
            _ => {
                Err(result::Error::BlockingApiError {
                    status_code,
                    method: "".to_string(),
                    url: url.to_string(),
                    response,
                })
            }
        }
    }
    pub fn list_api_project_members(
        &self,
        request: ListApiProjectMembersRequest,
    ) -> result::Result<Vec<ProjectMember>, error_enums::ListApiProjectMembersErrors> {
        let endpoint = format!("/api_project/{}/members", request.project_id_or_name);
        let url = format!("{}{}", self.base_url, endpoint);
        let query_params: Vec<(&str, String)> = vec![];
        let unauthed_builder = ReqwestClient::default().get(&url).query(&query_params);
        let authed_builder = self
            .builder_with_auth(unauthed_builder, &["ApiKeyAuth", "CookieAuth"]);
        let response = authed_builder.send().map_err(result::Error::Dispatch)?;
        let status_code = response.status().as_u16();
        match status_code {
            200 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Vec<ProjectMember>>(&response_text)
                    .unwrap();
                Ok(data)
            }
            _ => {
                Err(result::Error::BlockingApiError {
                    status_code,
                    method: "".to_string(),
                    url: url.to_string(),
                    response,
                })
            }
        }
    }
    pub fn list_api_versions(
        &self,
        request: ListApiVersionsRequest,
    ) -> result::Result<Vec<ApiVersion>, error_enums::ListApiVersionsErrors> {
        let endpoint = format!("/api_project/{}/version", request.project_id_or_name);
        let url = format!("{}{}", self.base_url, endpoint);
        let query_params: Vec<(&str, String)> = vec![];
        let unauthed_builder = ReqwestClient::default().get(&url).query(&query_params);
        let authed_builder = self
            .builder_with_auth(unauthed_builder, &["ApiKeyAuth", "CookieAuth"]);
        let response = authed_builder.send().map_err(result::Error::Dispatch)?;
        let status_code = response.status().as_u16();
        match status_code {
            200 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Vec<ApiVersion>>(&response_text)
                    .unwrap();
                Ok(data)
            }
            _ => {
                Err(result::Error::BlockingApiError {
                    status_code,
                    method: "".to_string(),
                    url: url.to_string(),
                    response,
                })
            }
        }
    }
    pub fn get_api_version(
        &self,
        request: GetApiVersionRequest,
    ) -> result::Result<ApiVersion, error_enums::GetApiVersionErrors> {
        let endpoint = format!(
            "/api_project/{}/version/{}", request.project_id_or_name, request
            .version_id_or_semver
        );
        let url = format!("{}{}", self.base_url, endpoint);
        let query_params: Vec<(&str, String)> = vec![];
        let unauthed_builder = ReqwestClient::default().get(&url).query(&query_params);
        let authed_builder = self
            .builder_with_auth(unauthed_builder, &["ApiKeyAuth", "CookieAuth"]);
        let response = authed_builder.send().map_err(result::Error::Dispatch)?;
        let status_code = response.status().as_u16();
        match status_code {
            200 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<ApiVersion>(&response_text).unwrap();
                Ok(data)
            }
            _ => {
                Err(result::Error::BlockingApiError {
                    status_code,
                    method: "".to_string(),
                    url: url.to_string(),
                    response,
                })
            }
        }
    }
    pub fn get_api_version_openapi(
        &self,
        request: GetApiVersionOpenapiRequest,
    ) -> result::Result<OpenApi, error_enums::GetApiVersionOpenapiErrors> {
        let endpoint = format!(
            "/api_project/{}/version/{}/openapi", request.project_id_or_name, request
            .version_id_or_semver
        );
        let url = format!("{}{}", self.base_url, endpoint);
        let query_params: Vec<(&str, String)> = vec![];
        let unauthed_builder = ReqwestClient::default().get(&url).query(&query_params);
        let authed_builder = self
            .builder_with_auth(unauthed_builder, &["ApiKeyAuth", "CookieAuth"]);
        let response = authed_builder.send().map_err(result::Error::Dispatch)?;
        let status_code = response.status().as_u16();
        match status_code {
            200 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<OpenApi>(&response_text).unwrap();
                Ok(data)
            }
            _ => {
                Err(result::Error::BlockingApiError {
                    status_code,
                    method: "".to_string(),
                    url: url.to_string(),
                    response,
                })
            }
        }
    }
    pub fn get_api_version_stats(
        &self,
        request: GetApiVersionStatsRequest,
    ) -> result::Result<Stats, error_enums::GetApiVersionStatsErrors> {
        let endpoint = format!(
            "/api_project/{}/version/{}/stats", request.project_id_or_name, request
            .version_id_or_semver
        );
        let url = format!("{}{}", self.base_url, endpoint);
        let query_params: Vec<(&str, String)> = vec![];
        let unauthed_builder = ReqwestClient::default().get(&url).query(&query_params);
        let authed_builder = self
            .builder_with_auth(unauthed_builder, &["ApiKeyAuth", "CookieAuth"]);
        let response = authed_builder.send().map_err(result::Error::Dispatch)?;
        let status_code = response.status().as_u16();
        match status_code {
            200 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Stats>(&response_text).unwrap();
                Ok(data)
            }
            _ => {
                Err(result::Error::BlockingApiError {
                    status_code,
                    method: "".to_string(),
                    url: url.to_string(),
                    response,
                })
            }
        }
    }
    pub fn exchange_code_for_key(
        &self,
        request: ExchangeCodeForKeyRequest,
    ) -> result::Result<UserApiKey, error_enums::ExchangeCodeForKeyErrors> {
        let endpoint = "/auth/exchange_key";
        let url = format!("{}{}", self.base_url, endpoint);
        let mut query_params: Vec<(&str, String)> = vec![];
        query_params.push(("code", request.code.to_string()));
        let unauthed_builder = ReqwestClient::default().get(&url).query(&query_params);
        let authed_builder = self.builder_with_auth(unauthed_builder, &[]);
        let response = authed_builder.send().map_err(result::Error::Dispatch)?;
        let status_code = response.status().as_u16();
        match status_code {
            200 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<UserApiKey>(&response_text).unwrap();
                Ok(data)
            }
            _ => {
                Err(result::Error::BlockingApiError {
                    status_code,
                    method: "".to_string(),
                    url: url.to_string(),
                    response,
                })
            }
        }
    }
    pub fn login_callback(
        &self,
        request: LoginCallbackRequest,
    ) -> result::Result<serde_json::Value, error_enums::LoginCallbackErrors> {
        let endpoint = "/auth/login_callback";
        let url = format!("{}{}", self.base_url, endpoint);
        let mut query_params: Vec<(&str, String)> = vec![];
        query_params.push(("code", request.code.to_string()));
        if let Some(state) = request.state {
            query_params.push(("state", state.to_string()));
        }
        let unauthed_builder = ReqwestClient::default().get(&url).query(&query_params);
        let authed_builder = self.builder_with_auth(unauthed_builder, &[]);
        let response = authed_builder.send().map_err(result::Error::Dispatch)?;
        let status_code = response.status().as_u16();
        match status_code {
            303 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<serde_json::Value>(&response_text)
                    .unwrap();
                Ok(data)
            }
            401 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "GET".to_string(),
                    url: url.to_string(),
                    data: error_enums::LoginCallbackErrors::Status401(data),
                })
            }
            _ => {
                Err(result::Error::BlockingApiError {
                    status_code,
                    method: "".to_string(),
                    url: url.to_string(),
                    response,
                })
            }
        }
    }
    pub fn login_url(
        &self,
        request: LoginUrlRequest,
    ) -> result::Result<serde_json::Value, error_enums::LoginUrlErrors> {
        let endpoint = "/auth/login_url";
        let url = format!("{}{}", self.base_url, endpoint);
        let mut query_params: Vec<(&str, String)> = vec![];
        if let Some(cli_output) = request.cli_output {
            query_params.push(("cli_output", cli_output.to_string()));
        }
        if let Some(cli_port) = request.cli_port {
            query_params.push(("cli_port", cli_port.to_string()));
        }
        let unauthed_builder = ReqwestClient::default().get(&url).query(&query_params);
        let authed_builder = self.builder_with_auth(unauthed_builder, &[]);
        let response = authed_builder.send().map_err(result::Error::Dispatch)?;
        let status_code = response.status().as_u16();
        match status_code {
            303 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<serde_json::Value>(&response_text)
                    .unwrap();
                Ok(data)
            }
            _ => {
                Err(result::Error::BlockingApiError {
                    status_code,
                    method: "".to_string(),
                    url: url.to_string(),
                    response,
                })
            }
        }
    }
    pub fn cli_check_updates(
        &self,
        request: CliCheckUpdatesRequest,
    ) -> result::Result<Vec<CliUpdate>, error_enums::CliCheckUpdatesErrors> {
        let endpoint = format!("/cli/updates/{}", request.cli_version);
        let url = format!("{}{}", self.base_url, endpoint);
        let query_params: Vec<(&str, String)> = vec![];
        let unauthed_builder = ReqwestClient::default().get(&url).query(&query_params);
        let authed_builder = self.builder_with_auth(unauthed_builder, &[]);
        let response = authed_builder.send().map_err(result::Error::Dispatch)?;
        let status_code = response.status().as_u16();
        match status_code {
            200 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Vec<CliUpdate>>(&response_text)
                    .unwrap();
                Ok(data)
            }
            _ => {
                Err(result::Error::BlockingApiError {
                    status_code,
                    method: "".to_string(),
                    url: url.to_string(),
                    response,
                })
            }
        }
    }
    pub fn list_doc_projects(
        &self,
    ) -> result::Result<Vec<DocProject>, error_enums::ListDocProjectsErrors> {
        let endpoint = "/doc_project";
        let url = format!("{}{}", self.base_url, endpoint);
        let query_params: Vec<(&str, String)> = vec![];
        let unauthed_builder = ReqwestClient::default().get(&url).query(&query_params);
        let authed_builder = self
            .builder_with_auth(unauthed_builder, &["ApiKeyAuth", "CookieAuth"]);
        let response = authed_builder.send().map_err(result::Error::Dispatch)?;
        let status_code = response.status().as_u16();
        match status_code {
            200 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Vec<DocProject>>(&response_text)
                    .unwrap();
                Ok(data)
            }
            _ => {
                Err(result::Error::BlockingApiError {
                    status_code,
                    method: "".to_string(),
                    url: url.to_string(),
                    response,
                })
            }
        }
    }
    pub fn get_doc_project(
        &self,
        request: GetDocProjectRequest,
    ) -> result::Result<DocProject, error_enums::GetDocProjectErrors> {
        let endpoint = format!("/doc_project/{}", request.project_id_or_name);
        let url = format!("{}{}", self.base_url, endpoint);
        let query_params: Vec<(&str, String)> = vec![];
        let unauthed_builder = ReqwestClient::default().get(&url).query(&query_params);
        let authed_builder = self
            .builder_with_auth(unauthed_builder, &["ApiKeyAuth", "CookieAuth"]);
        let response = authed_builder.send().map_err(result::Error::Dispatch)?;
        let status_code = response.status().as_u16();
        match status_code {
            200 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<DocProject>(&response_text).unwrap();
                Ok(data)
            }
            _ => {
                Err(result::Error::BlockingApiError {
                    status_code,
                    method: "".to_string(),
                    url: url.to_string(),
                    response,
                })
            }
        }
    }
    /// Retrieves all deployments for a doc project
    pub fn list_deployments(
        &self,
        request: ListDeploymentsRequest,
    ) -> result::Result<Vec<Deployment>, error_enums::ListDeploymentsErrors> {
        let endpoint = format!("/doc_project/{}/deployment", request.project_id_or_name);
        let url = format!("{}{}", self.base_url, endpoint);
        let mut query_params: Vec<(&str, String)> = vec![];
        if let Some(limit) = request.limit {
            query_params.push(("limit", limit.to_string()));
        }
        if let Some(target) = request.target {
            query_params.push(("target", target.to_string()));
        }
        let unauthed_builder = ReqwestClient::default().get(&url).query(&query_params);
        let authed_builder = self
            .builder_with_auth(unauthed_builder, &["ApiKeyAuth", "CookieAuth"]);
        let response = authed_builder.send().map_err(result::Error::Dispatch)?;
        let status_code = response.status().as_u16();
        match status_code {
            200 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Vec<Deployment>>(&response_text)
                    .unwrap();
                Ok(data)
            }
            _ => {
                Err(result::Error::BlockingApiError {
                    status_code,
                    method: "".to_string(),
                    url: url.to_string(),
                    response,
                })
            }
        }
    }
    /// Retrieves single deployment
    pub fn get_deployment(
        &self,
        request: GetDeploymentRequest,
    ) -> result::Result<Deployment, error_enums::GetDeploymentErrors> {
        let endpoint = format!(
            "/doc_project/{}/deployment/{}", request.project_id_or_name, request
            .deployment_id
        );
        let url = format!("{}{}", self.base_url, endpoint);
        let query_params: Vec<(&str, String)> = vec![];
        let unauthed_builder = ReqwestClient::default().get(&url).query(&query_params);
        let authed_builder = self
            .builder_with_auth(unauthed_builder, &["ApiKeyAuth", "CookieAuth"]);
        let response = authed_builder.send().map_err(result::Error::Dispatch)?;
        let status_code = response.status().as_u16();
        match status_code {
            200 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Deployment>(&response_text).unwrap();
                Ok(data)
            }
            _ => {
                Err(result::Error::BlockingApiError {
                    status_code,
                    method: "".to_string(),
                    url: url.to_string(),
                    response,
                })
            }
        }
    }
    pub fn list_doc_project_members(
        &self,
        request: ListDocProjectMembersRequest,
    ) -> result::Result<Vec<ProjectMember>, error_enums::ListDocProjectMembersErrors> {
        let endpoint = format!("/doc_project/{}/members", request.project_id_or_name);
        let url = format!("{}{}", self.base_url, endpoint);
        let query_params: Vec<(&str, String)> = vec![];
        let unauthed_builder = ReqwestClient::default().get(&url).query(&query_params);
        let authed_builder = self
            .builder_with_auth(unauthed_builder, &["ApiKeyAuth", "CookieAuth"]);
        let response = authed_builder.send().map_err(result::Error::Dispatch)?;
        let status_code = response.status().as_u16();
        match status_code {
            200 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Vec<ProjectMember>>(&response_text)
                    .unwrap();
                Ok(data)
            }
            _ => {
                Err(result::Error::BlockingApiError {
                    status_code,
                    method: "".to_string(),
                    url: url.to_string(),
                    response,
                })
            }
        }
    }
    pub fn check_preview(
        &self,
        request: CheckPreviewRequest,
    ) -> result::Result<bool, error_enums::CheckPreviewErrors> {
        let endpoint = format!("/doc_project/{}/preview", request.project_id_or_name);
        let url = format!("{}{}", self.base_url, endpoint);
        let query_params: Vec<(&str, String)> = vec![];
        let unauthed_builder = ReqwestClient::default().get(&url).query(&query_params);
        let authed_builder = self
            .builder_with_auth(unauthed_builder, &["ApiKeyAuth", "CookieAuth"]);
        let response = authed_builder.send().map_err(result::Error::Dispatch)?;
        let status_code = response.status().as_u16();
        match status_code {
            200 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<bool>(&response_text).unwrap();
                Ok(data)
            }
            _ => {
                Err(result::Error::BlockingApiError {
                    status_code,
                    method: "".to_string(),
                    url: url.to_string(),
                    response,
                })
            }
        }
    }
    pub fn get_doc_project_theme(
        &self,
        request: GetDocProjectThemeRequest,
    ) -> result::Result<Theme, error_enums::GetDocProjectThemeErrors> {
        let endpoint = format!("/doc_project/{}/theme", request.project_id_or_name);
        let url = format!("{}{}", self.base_url, endpoint);
        let query_params: Vec<(&str, String)> = vec![];
        let unauthed_builder = ReqwestClient::default().get(&url).query(&query_params);
        let authed_builder = self
            .builder_with_auth(unauthed_builder, &["ApiKeyAuth", "CookieAuth"]);
        let response = authed_builder.send().map_err(result::Error::Dispatch)?;
        let status_code = response.status().as_u16();
        match status_code {
            200 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Theme>(&response_text).unwrap();
                Ok(data)
            }
            _ => {
                Err(result::Error::BlockingApiError {
                    status_code,
                    method: "".to_string(),
                    url: url.to_string(),
                    response,
                })
            }
        }
    }
    pub fn list_doc_versions(
        &self,
        request: ListDocVersionsRequest,
    ) -> result::Result<Vec<DocVersion>, error_enums::ListDocVersionsErrors> {
        let endpoint = format!("/doc_project/{}/version", request.project_id_or_name);
        let url = format!("{}{}", self.base_url, endpoint);
        let query_params: Vec<(&str, String)> = vec![];
        let unauthed_builder = ReqwestClient::default().get(&url).query(&query_params);
        let authed_builder = self
            .builder_with_auth(unauthed_builder, &["ApiKeyAuth", "CookieAuth"]);
        let response = authed_builder.send().map_err(result::Error::Dispatch)?;
        let status_code = response.status().as_u16();
        match status_code {
            200 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Vec<DocVersion>>(&response_text)
                    .unwrap();
                Ok(data)
            }
            _ => {
                Err(result::Error::BlockingApiError {
                    status_code,
                    method: "".to_string(),
                    url: url.to_string(),
                    response,
                })
            }
        }
    }
    pub fn get_doc_version(
        &self,
        request: GetDocVersionRequest,
    ) -> result::Result<DocVersion, error_enums::GetDocVersionErrors> {
        let endpoint = format!(
            "/doc_project/{}/version/{}", request.project_id_or_name, request.version_id
        );
        let url = format!("{}{}", self.base_url, endpoint);
        let query_params: Vec<(&str, String)> = vec![];
        let unauthed_builder = ReqwestClient::default().get(&url).query(&query_params);
        let authed_builder = self
            .builder_with_auth(unauthed_builder, &["ApiKeyAuth", "CookieAuth"]);
        let response = authed_builder.send().map_err(result::Error::Dispatch)?;
        let status_code = response.status().as_u16();
        match status_code {
            200 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<DocVersion>(&response_text).unwrap();
                Ok(data)
            }
            _ => {
                Err(result::Error::BlockingApiError {
                    status_code,
                    method: "".to_string(),
                    url: url.to_string(),
                    response,
                })
            }
        }
    }
    pub fn list_guides(
        &self,
        request: ListGuidesRequest,
    ) -> result::Result<Vec<GuideWithChildren>, error_enums::ListGuidesErrors> {
        let endpoint = format!(
            "/doc_project/{}/version/{}/guide", request.project_id_or_name, request
            .version_id
        );
        let url = format!("{}{}", self.base_url, endpoint);
        let query_params: Vec<(&str, String)> = vec![];
        let unauthed_builder = ReqwestClient::default().get(&url).query(&query_params);
        let authed_builder = self
            .builder_with_auth(unauthed_builder, &["ApiKeyAuth", "CookieAuth"]);
        let response = authed_builder.send().map_err(result::Error::Dispatch)?;
        let status_code = response.status().as_u16();
        match status_code {
            200 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Vec<GuideWithChildren>>(&response_text)
                    .unwrap();
                Ok(data)
            }
            _ => {
                Err(result::Error::BlockingApiError {
                    status_code,
                    method: "".to_string(),
                    url: url.to_string(),
                    response,
                })
            }
        }
    }
    pub fn get_guide(
        &self,
        request: GetGuideRequest,
    ) -> result::Result<Guide, error_enums::GetGuideErrors> {
        let endpoint = format!(
            "/doc_project/{}/version/{}/guide/{}", request.project_id_or_name, request
            .version_id, request.guide_id
        );
        let url = format!("{}{}", self.base_url, endpoint);
        let query_params: Vec<(&str, String)> = vec![];
        let unauthed_builder = ReqwestClient::default().get(&url).query(&query_params);
        let authed_builder = self
            .builder_with_auth(unauthed_builder, &["ApiKeyAuth", "CookieAuth"]);
        let response = authed_builder.send().map_err(result::Error::Dispatch)?;
        let status_code = response.status().as_u16();
        match status_code {
            200 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Guide>(&response_text).unwrap();
                Ok(data)
            }
            _ => {
                Err(result::Error::BlockingApiError {
                    status_code,
                    method: "".to_string(),
                    url: url.to_string(),
                    response,
                })
            }
        }
    }
    pub fn get_guide_content(
        &self,
        request: GetGuideContentRequest,
    ) -> result::Result<GuideContent, error_enums::GetGuideContentErrors> {
        let endpoint = format!(
            "/doc_project/{}/version/{}/guide/{}/content", request.project_id_or_name,
            request.version_id, request.guide_id
        );
        let url = format!("{}{}", self.base_url, endpoint);
        let query_params: Vec<(&str, String)> = vec![];
        let unauthed_builder = ReqwestClient::default().get(&url).query(&query_params);
        let authed_builder = self
            .builder_with_auth(unauthed_builder, &["ApiKeyAuth", "CookieAuth"]);
        let response = authed_builder.send().map_err(result::Error::Dispatch)?;
        let status_code = response.status().as_u16();
        match status_code {
            200 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<GuideContent>(&response_text).unwrap();
                Ok(data)
            }
            _ => {
                Err(result::Error::BlockingApiError {
                    status_code,
                    method: "".to_string(),
                    url: url.to_string(),
                    response,
                })
            }
        }
    }
    /// Get user organization
    pub fn get_organization(
        &self,
    ) -> result::Result<Organization, error_enums::GetOrganizationErrors> {
        let endpoint = "/organization";
        let url = format!("{}{}", self.base_url, endpoint);
        let query_params: Vec<(&str, String)> = vec![];
        let unauthed_builder = ReqwestClient::default().get(&url).query(&query_params);
        let authed_builder = self
            .builder_with_auth(unauthed_builder, &["ApiKeyAuth", "CookieAuth"]);
        let response = authed_builder.send().map_err(result::Error::Dispatch)?;
        let status_code = response.status().as_u16();
        match status_code {
            200 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Organization>(&response_text).unwrap();
                Ok(data)
            }
            _ => {
                Err(result::Error::BlockingApiError {
                    status_code,
                    method: "".to_string(),
                    url: url.to_string(),
                    response,
                })
            }
        }
    }
    /// Get all assets for an organization
    pub fn get_assets(
        &self,
    ) -> result::Result<Vec<Asset>, error_enums::GetAssetsErrors> {
        let endpoint = "/organization/asset";
        let url = format!("{}{}", self.base_url, endpoint);
        let query_params: Vec<(&str, String)> = vec![];
        let unauthed_builder = ReqwestClient::default().get(&url).query(&query_params);
        let authed_builder = self
            .builder_with_auth(unauthed_builder, &["ApiKeyAuth", "CookieAuth"]);
        let response = authed_builder.send().map_err(result::Error::Dispatch)?;
        let status_code = response.status().as_u16();
        match status_code {
            200 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Vec<Asset>>(&response_text).unwrap();
                Ok(data)
            }
            _ => {
                Err(result::Error::BlockingApiError {
                    status_code,
                    method: "".to_string(),
                    url: url.to_string(),
                    response,
                })
            }
        }
    }
    /// Get users in the organization
    pub fn list_organization_members(
        &self,
    ) -> result::Result<
        Vec<OrganizationMember>,
        error_enums::ListOrganizationMembersErrors,
    > {
        let endpoint = "/organization/members";
        let url = format!("{}{}", self.base_url, endpoint);
        let query_params: Vec<(&str, String)> = vec![];
        let unauthed_builder = ReqwestClient::default().get(&url).query(&query_params);
        let authed_builder = self
            .builder_with_auth(unauthed_builder, &["ApiKeyAuth", "CookieAuth"]);
        let response = authed_builder.send().map_err(result::Error::Dispatch)?;
        let status_code = response.status().as_u16();
        match status_code {
            200 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<
                    Vec<OrganizationMember>,
                >(&response_text)
                    .unwrap();
                Ok(data)
            }
            _ => {
                Err(result::Error::BlockingApiError {
                    status_code,
                    method: "".to_string(),
                    url: url.to_string(),
                    response,
                })
            }
        }
    }
    /// Get documentation project theme configured at the organization level
    pub fn get_organization_theme(
        &self,
    ) -> result::Result<Theme, error_enums::GetOrganizationThemeErrors> {
        let endpoint = "/organization/theme";
        let url = format!("{}{}", self.base_url, endpoint);
        let query_params: Vec<(&str, String)> = vec![];
        let unauthed_builder = ReqwestClient::default().get(&url).query(&query_params);
        let authed_builder = self
            .builder_with_auth(unauthed_builder, &["ApiKeyAuth", "CookieAuth"]);
        let response = authed_builder.send().map_err(result::Error::Dispatch)?;
        let status_code = response.status().as_u16();
        match status_code {
            200 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Theme>(&response_text).unwrap();
                Ok(data)
            }
            _ => {
                Err(result::Error::BlockingApiError {
                    status_code,
                    method: "".to_string(),
                    url: url.to_string(),
                    response,
                })
            }
        }
    }
    pub fn get_current_user(
        &self,
    ) -> result::Result<User, error_enums::GetCurrentUserErrors> {
        let endpoint = "/user/me";
        let url = format!("{}{}", self.base_url, endpoint);
        let query_params: Vec<(&str, String)> = vec![];
        let unauthed_builder = ReqwestClient::default().get(&url).query(&query_params);
        let authed_builder = self
            .builder_with_auth(unauthed_builder, &["ApiKeyAuth", "CookieAuth"]);
        let response = authed_builder.send().map_err(result::Error::Dispatch)?;
        let status_code = response.status().as_u16();
        match status_code {
            200 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<User>(&response_text).unwrap();
                Ok(data)
            }
            _ => {
                Err(result::Error::BlockingApiError {
                    status_code,
                    method: "".to_string(),
                    url: url.to_string(),
                    response,
                })
            }
        }
    }
    pub fn get_api_key(
        &self,
    ) -> result::Result<UserApiKey, error_enums::GetApiKeyErrors> {
        let endpoint = "/user/me/api_key";
        let url = format!("{}{}", self.base_url, endpoint);
        let query_params: Vec<(&str, String)> = vec![];
        let unauthed_builder = ReqwestClient::default().get(&url).query(&query_params);
        let authed_builder = self
            .builder_with_auth(unauthed_builder, &["ApiKeyAuth", "CookieAuth"]);
        let response = authed_builder.send().map_err(result::Error::Dispatch)?;
        let status_code = response.status().as_u16();
        match status_code {
            200 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<UserApiKey>(&response_text).unwrap();
                Ok(data)
            }
            _ => {
                Err(result::Error::BlockingApiError {
                    status_code,
                    method: "".to_string(),
                    url: url.to_string(),
                    response,
                })
            }
        }
    }
    /// retrieve current user role for a given project type/id
    pub fn get_user_project_role(
        &self,
        request: GetUserProjectRoleRequest,
    ) -> result::Result<UserProjectRole, error_enums::GetUserProjectRoleErrors> {
        let endpoint = "/user/me/project_role";
        let url = format!("{}{}", self.base_url, endpoint);
        let mut query_params: Vec<(&str, String)> = vec![];
        query_params.push(("project_type", request.project_type.to_string()));
        if let Some(project_id_or_name) = request.project_id_or_name {
            query_params.push(("project_id_or_name", project_id_or_name.to_string()));
        }
        let unauthed_builder = ReqwestClient::default().get(&url).query(&query_params);
        let authed_builder = self
            .builder_with_auth(unauthed_builder, &["ApiKeyAuth", "CookieAuth"]);
        let response = authed_builder.send().map_err(result::Error::Dispatch)?;
        let status_code = response.status().as_u16();
        match status_code {
            200 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<UserProjectRole>(&response_text)
                    .unwrap();
                Ok(data)
            }
            _ => {
                Err(result::Error::BlockingApiError {
                    status_code,
                    method: "".to_string(),
                    url: url.to_string(),
                    response,
                })
            }
        }
    }
    pub fn get_service_accounts(
        &self,
    ) -> result::Result<Vec<ServiceAccount>, error_enums::GetServiceAccountsErrors> {
        let endpoint = "/user/service_account";
        let url = format!("{}{}", self.base_url, endpoint);
        let query_params: Vec<(&str, String)> = vec![];
        let unauthed_builder = ReqwestClient::default().get(&url).query(&query_params);
        let authed_builder = self
            .builder_with_auth(unauthed_builder, &["ApiKeyAuth", "CookieAuth"]);
        let response = authed_builder.send().map_err(result::Error::Dispatch)?;
        let status_code = response.status().as_u16();
        match status_code {
            200 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Vec<ServiceAccount>>(&response_text)
                    .unwrap();
                Ok(data)
            }
            _ => {
                Err(result::Error::BlockingApiError {
                    status_code,
                    method: "".to_string(),
                    url: url.to_string(),
                    response,
                })
            }
        }
    }
    pub fn update_api_link(
        &self,
        request: UpdateApiLinkRequest,
    ) -> result::Result<ApiLink, error_enums::UpdateApiLinkErrors> {
        let endpoint = format!("/api_link/{}", request.link_id);
        let url = format!("{}{}", self.base_url, endpoint);
        let query_params: Vec<(&str, String)> = vec![];
        let unauthed_builder = ReqwestClient::default().patch(&url).query(&query_params);
        let authed_builder = self
            .builder_with_auth(unauthed_builder, &["ApiKeyAuth", "CookieAuth"]);
        let request_body: serde_json::Value = serde_json::to_value(request.data)
            .map_err(result::Error::Serialize)?;
        let response = authed_builder
            .json(&request_body)
            .send()
            .map_err(result::Error::Dispatch)?;
        let status_code = response.status().as_u16();
        match status_code {
            200 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<ApiLink>(&response_text).unwrap();
                Ok(data)
            }
            _ => {
                Err(result::Error::BlockingApiError {
                    status_code,
                    method: "".to_string(),
                    url: url.to_string(),
                    response,
                })
            }
        }
    }
    pub fn update_api_link_group(
        &self,
        request: UpdateApiLinkGroupRequest,
    ) -> result::Result<ApiLinkGroup, error_enums::UpdateApiLinkGroupErrors> {
        let endpoint = format!("/api_link_group/{}", request.group_id);
        let url = format!("{}{}", self.base_url, endpoint);
        let query_params: Vec<(&str, String)> = vec![];
        let unauthed_builder = ReqwestClient::default().patch(&url).query(&query_params);
        let authed_builder = self
            .builder_with_auth(unauthed_builder, &["ApiKeyAuth", "CookieAuth"]);
        let request_body: serde_json::Value = serde_json::to_value(request.data)
            .map_err(result::Error::Serialize)?;
        let response = authed_builder
            .json(&request_body)
            .send()
            .map_err(result::Error::Dispatch)?;
        let status_code = response.status().as_u16();
        match status_code {
            200 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<ApiLinkGroup>(&response_text).unwrap();
                Ok(data)
            }
            _ => {
                Err(result::Error::BlockingApiError {
                    status_code,
                    method: "".to_string(),
                    url: url.to_string(),
                    response,
                })
            }
        }
    }
    pub fn update_api_project(
        &self,
        request: UpdateApiProjectRequest,
    ) -> result::Result<ApiProject, error_enums::UpdateApiProjectErrors> {
        let endpoint = format!("/api_project/{}", request.project_id_or_name);
        let url = format!("{}{}", self.base_url, endpoint);
        let query_params: Vec<(&str, String)> = vec![];
        let unauthed_builder = ReqwestClient::default().patch(&url).query(&query_params);
        let authed_builder = self
            .builder_with_auth(unauthed_builder, &["ApiKeyAuth", "CookieAuth"]);
        let request_body: serde_json::Value = serde_json::to_value(request.data)
            .map_err(result::Error::Serialize)?;
        let response = authed_builder
            .json(&request_body)
            .send()
            .map_err(result::Error::Dispatch)?;
        let status_code = response.status().as_u16();
        match status_code {
            200 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<ApiProject>(&response_text).unwrap();
                Ok(data)
            }
            _ => {
                Err(result::Error::BlockingApiError {
                    status_code,
                    method: "".to_string(),
                    url: url.to_string(),
                    response,
                })
            }
        }
    }
    pub fn update_api_version(
        &self,
        request: UpdateApiVersionRequest,
    ) -> result::Result<ApiVersion, error_enums::UpdateApiVersionErrors> {
        let endpoint = format!(
            "/api_project/{}/version/{}", request.project_id_or_name, request
            .version_id_or_semver
        );
        let url = format!("{}{}", self.base_url, endpoint);
        let query_params: Vec<(&str, String)> = vec![];
        let unauthed_builder = ReqwestClient::default().patch(&url).query(&query_params);
        let authed_builder = self
            .builder_with_auth(unauthed_builder, &["ApiKeyAuth", "CookieAuth"]);
        let request_body: serde_json::Value = serde_json::to_value(request.data)
            .map_err(result::Error::Serialize)?;
        let response = authed_builder
            .json(&request_body)
            .send()
            .map_err(result::Error::Dispatch)?;
        let status_code = response.status().as_u16();
        match status_code {
            200 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<ApiVersion>(&response_text).unwrap();
                Ok(data)
            }
            _ => {
                Err(result::Error::BlockingApiError {
                    status_code,
                    method: "".to_string(),
                    url: url.to_string(),
                    response,
                })
            }
        }
    }
    pub fn update_doc_project(
        &self,
        request: UpdateDocProjectRequest,
    ) -> result::Result<DocProject, error_enums::UpdateDocProjectErrors> {
        let endpoint = format!("/doc_project/{}", request.project_id_or_name);
        let url = format!("{}{}", self.base_url, endpoint);
        let query_params: Vec<(&str, String)> = vec![];
        let unauthed_builder = ReqwestClient::default().patch(&url).query(&query_params);
        let authed_builder = self
            .builder_with_auth(unauthed_builder, &["ApiKeyAuth", "CookieAuth"]);
        let request_body: serde_json::Value = serde_json::to_value(request.data)
            .map_err(result::Error::Serialize)?;
        let response = authed_builder
            .json(&request_body)
            .send()
            .map_err(result::Error::Dispatch)?;
        let status_code = response.status().as_u16();
        match status_code {
            200 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<DocProject>(&response_text).unwrap();
                Ok(data)
            }
            _ => {
                Err(result::Error::BlockingApiError {
                    status_code,
                    method: "".to_string(),
                    url: url.to_string(),
                    response,
                })
            }
        }
    }
    pub fn update_guide(
        &self,
        request: UpdateGuideRequest,
    ) -> result::Result<Guide, error_enums::UpdateGuideErrors> {
        let endpoint = format!(
            "/doc_project/{}/version/{}/guide/{}", request.project_id_or_name, request
            .version_id, request.guide_id
        );
        let url = format!("{}{}", self.base_url, endpoint);
        let query_params: Vec<(&str, String)> = vec![];
        let unauthed_builder = ReqwestClient::default().patch(&url).query(&query_params);
        let authed_builder = self
            .builder_with_auth(unauthed_builder, &["ApiKeyAuth", "CookieAuth"]);
        let request_body: serde_json::Value = serde_json::to_value(request.data)
            .map_err(result::Error::Serialize)?;
        let response = authed_builder
            .json(&request_body)
            .send()
            .map_err(result::Error::Dispatch)?;
        let status_code = response.status().as_u16();
        match status_code {
            200 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Guide>(&response_text).unwrap();
                Ok(data)
            }
            _ => {
                Err(result::Error::BlockingApiError {
                    status_code,
                    method: "".to_string(),
                    url: url.to_string(),
                    response,
                })
            }
        }
    }
    pub fn create_api_link(
        &self,
        request: CreateApiLinkRequest,
    ) -> result::Result<ApiLink, error_enums::CreateApiLinkErrors> {
        let endpoint = "/api_link";
        let url = format!("{}{}", self.base_url, endpoint);
        let query_params: Vec<(&str, String)> = vec![];
        let unauthed_builder = ReqwestClient::default().post(&url).query(&query_params);
        let authed_builder = self
            .builder_with_auth(unauthed_builder, &["ApiKeyAuth", "CookieAuth"]);
        let request_body: serde_json::Value = serde_json::to_value(request.data)
            .map_err(result::Error::Serialize)?;
        let response = authed_builder
            .json(&request_body)
            .send()
            .map_err(result::Error::Dispatch)?;
        let status_code = response.status().as_u16();
        match status_code {
            201 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<ApiLink>(&response_text).unwrap();
                Ok(data)
            }
            _ => {
                Err(result::Error::BlockingApiError {
                    status_code,
                    method: "".to_string(),
                    url: url.to_string(),
                    response,
                })
            }
        }
    }
    pub fn reorder_api_links(
        &self,
        request: ReorderApiLinksRequest,
    ) -> result::Result<ApiReorder, error_enums::ReorderApiLinksErrors> {
        let endpoint = "/api_link/reorder";
        let url = format!("{}{}", self.base_url, endpoint);
        let query_params: Vec<(&str, String)> = vec![];
        let unauthed_builder = ReqwestClient::default().post(&url).query(&query_params);
        let authed_builder = self
            .builder_with_auth(unauthed_builder, &["ApiKeyAuth", "CookieAuth"]);
        let request_body: serde_json::Value = serde_json::to_value(request.data)
            .map_err(result::Error::Serialize)?;
        let response = authed_builder
            .json(&request_body)
            .send()
            .map_err(result::Error::Dispatch)?;
        let status_code = response.status().as_u16();
        match status_code {
            200 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<ApiReorder>(&response_text).unwrap();
                Ok(data)
            }
            _ => {
                Err(result::Error::BlockingApiError {
                    status_code,
                    method: "".to_string(),
                    url: url.to_string(),
                    response,
                })
            }
        }
    }
    pub fn create_api_link_group(
        &self,
        request: CreateApiLinkGroupRequest,
    ) -> result::Result<ApiLinkGroup, error_enums::CreateApiLinkGroupErrors> {
        let endpoint = "/api_link_group";
        let url = format!("{}{}", self.base_url, endpoint);
        let query_params: Vec<(&str, String)> = vec![];
        let unauthed_builder = ReqwestClient::default().post(&url).query(&query_params);
        let authed_builder = self
            .builder_with_auth(unauthed_builder, &["ApiKeyAuth", "CookieAuth"]);
        let request_body: serde_json::Value = serde_json::to_value(request.data)
            .map_err(result::Error::Serialize)?;
        let response = authed_builder
            .json(&request_body)
            .send()
            .map_err(result::Error::Dispatch)?;
        let status_code = response.status().as_u16();
        match status_code {
            201 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<ApiLinkGroup>(&response_text).unwrap();
                Ok(data)
            }
            _ => {
                Err(result::Error::BlockingApiError {
                    status_code,
                    method: "".to_string(),
                    url: url.to_string(),
                    response,
                })
            }
        }
    }
    pub fn create_api_project(
        &self,
        request: CreateApiProjectRequest,
    ) -> result::Result<ApiProject, error_enums::CreateApiProjectErrors> {
        let endpoint = "/api_project";
        let url = format!("{}{}", self.base_url, endpoint);
        let query_params: Vec<(&str, String)> = vec![];
        let unauthed_builder = ReqwestClient::default().post(&url).query(&query_params);
        let authed_builder = self
            .builder_with_auth(unauthed_builder, &["ApiKeyAuth", "CookieAuth"]);
        let request_body: serde_json::Value = serde_json::to_value(request.data)
            .map_err(result::Error::Serialize)?;
        let response = authed_builder
            .json(&request_body)
            .send()
            .map_err(result::Error::Dispatch)?;
        let status_code = response.status().as_u16();
        match status_code {
            201 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<ApiProject>(&response_text).unwrap();
                Ok(data)
            }
            _ => {
                Err(result::Error::BlockingApiError {
                    status_code,
                    method: "".to_string(),
                    url: url.to_string(),
                    response,
                })
            }
        }
    }
    pub fn grant_api_project_role(
        &self,
        request: GrantApiProjectRoleRequest,
    ) -> result::Result<serde_json::Value, error_enums::GrantApiProjectRoleErrors> {
        let endpoint = format!("/api_project/{}/role", request.project_id_or_name);
        let url = format!("{}{}", self.base_url, endpoint);
        let query_params: Vec<(&str, String)> = vec![];
        let unauthed_builder = ReqwestClient::default().post(&url).query(&query_params);
        let authed_builder = self
            .builder_with_auth(unauthed_builder, &["ApiKeyAuth", "CookieAuth"]);
        let request_body: serde_json::Value = serde_json::to_value(request.data)
            .map_err(result::Error::Serialize)?;
        let response = authed_builder
            .json(&request_body)
            .send()
            .map_err(result::Error::Dispatch)?;
        let status_code = response.status().as_u16();
        match status_code {
            201 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<serde_json::Value>(&response_text)
                    .unwrap();
                Ok(data)
            }
            _ => {
                Err(result::Error::BlockingApiError {
                    status_code,
                    method: "".to_string(),
                    url: url.to_string(),
                    response,
                })
            }
        }
    }
    pub fn create_api_version(
        &self,
        request: CreateApiVersionRequest,
    ) -> result::Result<ApiVersion, error_enums::CreateApiVersionErrors> {
        let endpoint = format!("/api_project/{}/version", request.project_id_or_name);
        let url = format!("{}{}", self.base_url, endpoint);
        let query_params: Vec<(&str, String)> = vec![];
        let unauthed_builder = ReqwestClient::default().post(&url).query(&query_params);
        let authed_builder = self
            .builder_with_auth(unauthed_builder, &["ApiKeyAuth", "CookieAuth"]);
        let request_body: serde_json::Value = serde_json::to_value(request.data)
            .map_err(result::Error::Serialize)?;
        let response = authed_builder
            .json(&request_body)
            .send()
            .map_err(result::Error::Dispatch)?;
        let status_code = response.status().as_u16();
        match status_code {
            201 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<ApiVersion>(&response_text).unwrap();
                Ok(data)
            }
            _ => {
                Err(result::Error::BlockingApiError {
                    status_code,
                    method: "".to_string(),
                    url: url.to_string(),
                    response,
                })
            }
        }
    }
    pub fn create_doc_project(
        &self,
        request: CreateDocProjectRequest,
    ) -> result::Result<DocProject, error_enums::CreateDocProjectErrors> {
        let endpoint = "/doc_project";
        let url = format!("{}{}", self.base_url, endpoint);
        let query_params: Vec<(&str, String)> = vec![];
        let unauthed_builder = ReqwestClient::default().post(&url).query(&query_params);
        let authed_builder = self
            .builder_with_auth(unauthed_builder, &["ApiKeyAuth", "CookieAuth"]);
        let request_body: serde_json::Value = serde_json::to_value(request.data)
            .map_err(result::Error::Serialize)?;
        let response = authed_builder
            .json(&request_body)
            .send()
            .map_err(result::Error::Dispatch)?;
        let status_code = response.status().as_u16();
        match status_code {
            201 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<DocProject>(&response_text).unwrap();
                Ok(data)
            }
            _ => {
                Err(result::Error::BlockingApiError {
                    status_code,
                    method: "".to_string(),
                    url: url.to_string(),
                    response,
                })
            }
        }
    }
    /// Deploys a new generated version of documentation with linked guides & APIs
    pub fn trigger_deployment(
        &self,
        request: TriggerDeploymentRequest,
    ) -> result::Result<Deployment, error_enums::TriggerDeploymentErrors> {
        let endpoint = format!("/doc_project/{}/deployment", request.project_id_or_name);
        let url = format!("{}{}", self.base_url, endpoint);
        let query_params: Vec<(&str, String)> = vec![];
        let unauthed_builder = ReqwestClient::default().post(&url).query(&query_params);
        let authed_builder = self
            .builder_with_auth(unauthed_builder, &["ApiKeyAuth", "CookieAuth"]);
        let request_body: serde_json::Value = serde_json::to_value(request.data)
            .map_err(result::Error::Serialize)?;
        let response = authed_builder
            .json(&request_body)
            .send()
            .map_err(result::Error::Dispatch)?;
        let status_code = response.status().as_u16();
        match status_code {
            201 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Deployment>(&response_text).unwrap();
                Ok(data)
            }
            _ => {
                Err(result::Error::BlockingApiError {
                    status_code,
                    method: "".to_string(),
                    url: url.to_string(),
                    response,
                })
            }
        }
    }
    pub fn grant_doc_project_role(
        &self,
        request: GrantDocProjectRoleRequest,
    ) -> result::Result<serde_json::Value, error_enums::GrantDocProjectRoleErrors> {
        let endpoint = format!("/doc_project/{}/role", request.project_id_or_name);
        let url = format!("{}{}", self.base_url, endpoint);
        let query_params: Vec<(&str, String)> = vec![];
        let unauthed_builder = ReqwestClient::default().post(&url).query(&query_params);
        let authed_builder = self
            .builder_with_auth(unauthed_builder, &["ApiKeyAuth", "CookieAuth"]);
        let request_body: serde_json::Value = serde_json::to_value(request.data)
            .map_err(result::Error::Serialize)?;
        let response = authed_builder
            .json(&request_body)
            .send()
            .map_err(result::Error::Dispatch)?;
        let status_code = response.status().as_u16();
        match status_code {
            201 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<serde_json::Value>(&response_text)
                    .unwrap();
                Ok(data)
            }
            _ => {
                Err(result::Error::BlockingApiError {
                    status_code,
                    method: "".to_string(),
                    url: url.to_string(),
                    response,
                })
            }
        }
    }
    pub fn create_guide(
        &self,
        request: CreateGuideRequest,
    ) -> result::Result<Guide, error_enums::CreateGuideErrors> {
        let endpoint = format!(
            "/doc_project/{}/version/{}/guide", request.project_id_or_name, request
            .version_id
        );
        let url = format!("{}{}", self.base_url, endpoint);
        let query_params: Vec<(&str, String)> = vec![];
        let unauthed_builder = ReqwestClient::default().post(&url).query(&query_params);
        let authed_builder = self
            .builder_with_auth(unauthed_builder, &["ApiKeyAuth", "CookieAuth"]);
        let request_body: serde_json::Value = serde_json::to_value(request.data)
            .map_err(result::Error::Serialize)?;
        let response = authed_builder
            .json(&request_body)
            .send()
            .map_err(result::Error::Dispatch)?;
        let status_code = response.status().as_u16();
        match status_code {
            201 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Guide>(&response_text).unwrap();
                Ok(data)
            }
            _ => {
                Err(result::Error::BlockingApiError {
                    status_code,
                    method: "".to_string(),
                    url: url.to_string(),
                    response,
                })
            }
        }
    }
    pub fn reorder_guides(
        &self,
        request: ReorderGuidesRequest,
    ) -> result::Result<Vec<GuideWithChildren>, error_enums::ReorderGuidesErrors> {
        let endpoint = format!(
            "/doc_project/{}/version/{}/guide/reorder", request.project_id_or_name,
            request.version_id
        );
        let url = format!("{}{}", self.base_url, endpoint);
        let query_params: Vec<(&str, String)> = vec![];
        let unauthed_builder = ReqwestClient::default().post(&url).query(&query_params);
        let authed_builder = self
            .builder_with_auth(unauthed_builder, &["ApiKeyAuth", "CookieAuth"]);
        let request_body: serde_json::Value = serde_json::to_value(request.data)
            .map_err(result::Error::Serialize)?;
        let response = authed_builder
            .json(&request_body)
            .send()
            .map_err(result::Error::Dispatch)?;
        let status_code = response.status().as_u16();
        match status_code {
            200 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Vec<GuideWithChildren>>(&response_text)
                    .unwrap();
                Ok(data)
            }
            _ => {
                Err(result::Error::BlockingApiError {
                    status_code,
                    method: "".to_string(),
                    url: url.to_string(),
                    response,
                })
            }
        }
    }
    pub fn create_organization(
        &self,
        request: CreateOrganizationRequest,
    ) -> result::Result<
        OrganizationWithRedirect,
        error_enums::CreateOrganizationErrors,
    > {
        let endpoint = "/organization";
        let url = format!("{}{}", self.base_url, endpoint);
        let query_params: Vec<(&str, String)> = vec![];
        let unauthed_builder = ReqwestClient::default().post(&url).query(&query_params);
        let authed_builder = self
            .builder_with_auth(unauthed_builder, &["ApiKeyAuth", "CookieAuth"]);
        let request_body: serde_json::Value = serde_json::to_value(request.data)
            .map_err(result::Error::Serialize)?;
        let response = authed_builder
            .json(&request_body)
            .send()
            .map_err(result::Error::Dispatch)?;
        let status_code = response.status().as_u16();
        match status_code {
            201 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<
                    OrganizationWithRedirect,
                >(&response_text)
                    .unwrap();
                Ok(data)
            }
            _ => {
                Err(result::Error::BlockingApiError {
                    status_code,
                    method: "".to_string(),
                    url: url.to_string(),
                    response,
                })
            }
        }
    }
    /// Add a assets like logos to an organization
    pub fn upload_assets(
        &self,
        request: UploadAssetsRequest,
    ) -> result::Result<Vec<Asset>, error_enums::UploadAssetsErrors> {
        let endpoint = "/organization/asset";
        let url = format!("{}{}", self.base_url, endpoint);
        let query_params: Vec<(&str, String)> = vec![];
        let unauthed_builder = ReqwestClient::default().post(&url).query(&query_params);
        let authed_builder = self
            .builder_with_auth(unauthed_builder, &["ApiKeyAuth", "CookieAuth"]);
        let mut form_data_body = reqwest_multipart::Form::new();
        form_data_body = form_data_body
            .part(
                "file",
                reqwest_multipart::Part::file(&request.data.file)
                    .map_err(result::Error::File)?,
            );
        let response = authed_builder
            .multipart(form_data_body)
            .send()
            .map_err(result::Error::Dispatch)?;
        let status_code = response.status().as_u16();
        match status_code {
            200 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Vec<Asset>>(&response_text).unwrap();
                Ok(data)
            }
            _ => {
                Err(result::Error::BlockingApiError {
                    status_code,
                    method: "".to_string(),
                    url: url.to_string(),
                    response,
                })
            }
        }
    }
    pub fn create_sdk(
        &self,
        request: CreateSdkRequest,
    ) -> result::Result<BinaryResponse, error_enums::CreateSdkErrors> {
        let endpoint = "/sdk";
        let url = format!("{}{}", self.base_url, endpoint);
        let query_params: Vec<(&str, String)> = vec![];
        let unauthed_builder = ReqwestClient::default().post(&url).query(&query_params);
        let authed_builder = self
            .builder_with_auth(unauthed_builder, &["ApiKeyAuth", "CookieAuth"]);
        let request_body: serde_json::Value = serde_json::to_value(request.data)
            .map_err(result::Error::Serialize)?;
        let response = authed_builder
            .json(&request_body)
            .send()
            .map_err(result::Error::Dispatch)?;
        let status_code = response.status().as_u16();
        match status_code {
            201 => {
                let res_bytes = response.bytes().map_err(result::Error::ResponseBytes)?;
                let data = BinaryResponse {
                    content: res_bytes,
                };
                Ok(data)
            }
            _ => {
                Err(result::Error::BlockingApiError {
                    status_code,
                    method: "".to_string(),
                    url: url.to_string(),
                    response,
                })
            }
        }
    }
    pub fn update_sdk(
        &self,
        request: UpdateSdkRequest,
    ) -> result::Result<NewSdkResponse, error_enums::UpdateSdkErrors> {
        let endpoint = "/sdk/new_version";
        let url = format!("{}{}", self.base_url, endpoint);
        let query_params: Vec<(&str, String)> = vec![];
        let unauthed_builder = ReqwestClient::default().post(&url).query(&query_params);
        let authed_builder = self
            .builder_with_auth(unauthed_builder, &["ApiKeyAuth", "CookieAuth"]);
        let request_body: serde_json::Value = serde_json::to_value(request.data)
            .map_err(result::Error::Serialize)?;
        let response = authed_builder
            .json(&request_body)
            .send()
            .map_err(result::Error::Dispatch)?;
        let status_code = response.status().as_u16();
        match status_code {
            200 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<NewSdkResponse>(&response_text)
                    .unwrap();
                Ok(data)
            }
            _ => {
                Err(result::Error::BlockingApiError {
                    status_code,
                    method: "".to_string(),
                    url: url.to_string(),
                    response,
                })
            }
        }
    }
    pub fn stateless_generate_sdk(
        &self,
        request: StatelessGenerateSdkRequest,
    ) -> result::Result<BinaryResponse, error_enums::StatelessGenerateSdkErrors> {
        let endpoint = "/stateless/generate_sdk";
        let url = format!("{}{}", self.base_url, endpoint);
        let query_params: Vec<(&str, String)> = vec![];
        let unauthed_builder = ReqwestClient::default().post(&url).query(&query_params);
        let authed_builder = self
            .builder_with_auth(unauthed_builder, &["ApiKeyAuth", "CookieAuth"]);
        let request_body: serde_json::Value = serde_json::to_value(request.data)
            .map_err(result::Error::Serialize)?;
        let response = authed_builder
            .json(&request_body)
            .send()
            .map_err(result::Error::Dispatch)?;
        let status_code = response.status().as_u16();
        match status_code {
            201 => {
                let res_bytes = response.bytes().map_err(result::Error::ResponseBytes)?;
                let data = BinaryResponse {
                    content: res_bytes,
                };
                Ok(data)
            }
            _ => {
                Err(result::Error::BlockingApiError {
                    status_code,
                    method: "".to_string(),
                    url: url.to_string(),
                    response,
                })
            }
        }
    }
    pub fn invite_user(
        &self,
        request: InviteUserRequest,
    ) -> result::Result<serde_json::Value, error_enums::InviteUserErrors> {
        let endpoint = "/user/invite";
        let url = format!("{}{}", self.base_url, endpoint);
        let query_params: Vec<(&str, String)> = vec![];
        let unauthed_builder = ReqwestClient::default().post(&url).query(&query_params);
        let authed_builder = self
            .builder_with_auth(unauthed_builder, &["ApiKeyAuth", "CookieAuth"]);
        let request_body: serde_json::Value = serde_json::to_value(request.data)
            .map_err(result::Error::Serialize)?;
        let response = authed_builder
            .json(&request_body)
            .send()
            .map_err(result::Error::Dispatch)?;
        let status_code = response.status().as_u16();
        match status_code {
            201 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<serde_json::Value>(&response_text)
                    .unwrap();
                Ok(data)
            }
            _ => {
                Err(result::Error::BlockingApiError {
                    status_code,
                    method: "".to_string(),
                    url: url.to_string(),
                    response,
                })
            }
        }
    }
    pub fn create_service_account(
        &self,
        request: CreateServiceAccountRequest,
    ) -> result::Result<UserApiKey, error_enums::CreateServiceAccountErrors> {
        let endpoint = "/user/service_account";
        let url = format!("{}{}", self.base_url, endpoint);
        let query_params: Vec<(&str, String)> = vec![];
        let unauthed_builder = ReqwestClient::default().post(&url).query(&query_params);
        let authed_builder = self
            .builder_with_auth(unauthed_builder, &["ApiKeyAuth", "CookieAuth"]);
        let request_body: serde_json::Value = serde_json::to_value(request.data)
            .map_err(result::Error::Serialize)?;
        let response = authed_builder
            .json(&request_body)
            .send()
            .map_err(result::Error::Dispatch)?;
        let status_code = response.status().as_u16();
        match status_code {
            200 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<UserApiKey>(&response_text).unwrap();
                Ok(data)
            }
            _ => {
                Err(result::Error::BlockingApiError {
                    status_code,
                    method: "".to_string(),
                    url: url.to_string(),
                    response,
                })
            }
        }
    }
    pub fn vercel_webhook(
        &self,
        request: VercelWebhookRequest,
    ) -> result::Result<serde_json::Value, error_enums::VercelWebhookErrors> {
        let endpoint = "/webhook/vercel";
        let url = format!("{}{}", self.base_url, endpoint);
        let query_params: Vec<(&str, String)> = vec![];
        let unauthed_builder = ReqwestClient::default().post(&url).query(&query_params);
        let authed_builder = self
            .builder_with_auth(unauthed_builder, &["ApiKeyAuth", "CookieAuth"]);
        let request_body: serde_json::Value = serde_json::to_value(request.data)
            .map_err(result::Error::Serialize)?;
        let response = authed_builder
            .json(&request_body)
            .send()
            .map_err(result::Error::Dispatch)?;
        let status_code = response.status().as_u16();
        match status_code {
            202 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<serde_json::Value>(&response_text)
                    .unwrap();
                Ok(data)
            }
            _ => {
                Err(result::Error::BlockingApiError {
                    status_code,
                    method: "".to_string(),
                    url: url.to_string(),
                    response,
                })
            }
        }
    }
    pub fn update_doc_project_theme(
        &self,
        request: UpdateDocProjectThemeRequest,
    ) -> result::Result<Theme, error_enums::UpdateDocProjectThemeErrors> {
        let endpoint = format!("/doc_project/{}/theme", request.project_id_or_name);
        let url = format!("{}{}", self.base_url, endpoint);
        let query_params: Vec<(&str, String)> = vec![];
        let unauthed_builder = ReqwestClient::default().put(&url).query(&query_params);
        let authed_builder = self
            .builder_with_auth(unauthed_builder, &["ApiKeyAuth", "CookieAuth"]);
        let request_body: serde_json::Value = serde_json::to_value(request.data)
            .map_err(result::Error::Serialize)?;
        let response = authed_builder
            .json(&request_body)
            .send()
            .map_err(result::Error::Dispatch)?;
        let status_code = response.status().as_u16();
        match status_code {
            200 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Theme>(&response_text).unwrap();
                Ok(data)
            }
            _ => {
                Err(result::Error::BlockingApiError {
                    status_code,
                    method: "".to_string(),
                    url: url.to_string(),
                    response,
                })
            }
        }
    }
    /// Update  documentation project theme configured at the organization level
    pub fn update_organization_theme(
        &self,
        request: UpdateOrganizationThemeRequest,
    ) -> result::Result<Theme, error_enums::UpdateOrganizationThemeErrors> {
        let endpoint = "/organization/theme";
        let url = format!("{}{}", self.base_url, endpoint);
        let query_params: Vec<(&str, String)> = vec![];
        let unauthed_builder = ReqwestClient::default().put(&url).query(&query_params);
        let authed_builder = self
            .builder_with_auth(unauthed_builder, &["ApiKeyAuth", "CookieAuth"]);
        let request_body: serde_json::Value = serde_json::to_value(request.data)
            .map_err(result::Error::Serialize)?;
        let response = authed_builder
            .json(&request_body)
            .send()
            .map_err(result::Error::Dispatch)?;
        let status_code = response.status().as_u16();
        match status_code {
            200 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Theme>(&response_text).unwrap();
                Ok(data)
            }
            _ => {
                Err(result::Error::BlockingApiError {
                    status_code,
                    method: "".to_string(),
                    url: url.to_string(),
                    response,
                })
            }
        }
    }
}
#[cfg(test)]
mod tests {
    /// Generated by Sideko (sideko.dev)
    use super::*;
    #[test]
    fn test_delete_api_link_204_generated() {
        let client = Client::default()
            .with_api_key_auth(
                &std::env::var("API_KEY").expect("API key not defined"),
            )
            .with_cookie_auth(
                &std::env::var("API_KEY").expect("API key not defined"),
            );
        let response = client
            .delete_api_link(DeleteApiLinkRequest {
                link_id: "3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a".to_string(),
            });
        assert!(response.is_ok());
    }
    #[test]
    fn test_delete_api_link_group_204_generated() {
        let client = Client::default()
            .with_api_key_auth(
                &std::env::var("API_KEY").expect("API key not defined"),
            )
            .with_cookie_auth(
                &std::env::var("API_KEY").expect("API key not defined"),
            );
        let response = client
            .delete_api_link_group(DeleteApiLinkGroupRequest {
                group_id: "3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a".to_string(),
            });
        assert!(response.is_ok());
    }
    #[test]
    fn test_delete_api_project_204_generated() {
        let client = Client::default()
            .with_api_key_auth(
                &std::env::var("API_KEY").expect("API key not defined"),
            )
            .with_cookie_auth(
                &std::env::var("API_KEY").expect("API key not defined"),
            );
        let response = client
            .delete_api_project(DeleteApiProjectRequest {
                project_id_or_name: "string".to_string(),
            });
        assert!(response.is_ok());
    }
    #[test]
    fn test_delete_api_project_role_204_generated() {
        let client = Client::default()
            .with_api_key_auth(
                &std::env::var("API_KEY").expect("API key not defined"),
            )
            .with_cookie_auth(
                &std::env::var("API_KEY").expect("API key not defined"),
            );
        let response = client
            .delete_api_project_role(DeleteApiProjectRoleRequest {
                project_id_or_name: "string".to_string(),
                user_id: "string".to_string(),
            });
        assert!(response.is_ok());
    }
    #[test]
    fn test_delete_doc_project_204_generated() {
        let client = Client::default()
            .with_api_key_auth(
                &std::env::var("API_KEY").expect("API key not defined"),
            )
            .with_cookie_auth(
                &std::env::var("API_KEY").expect("API key not defined"),
            );
        let response = client
            .delete_doc_project(DeleteDocProjectRequest {
                project_id_or_name: "string".to_string(),
            });
        assert!(response.is_ok());
    }
    #[test]
    fn test_delete_doc_project_role_204_generated() {
        let client = Client::default()
            .with_api_key_auth(
                &std::env::var("API_KEY").expect("API key not defined"),
            )
            .with_cookie_auth(
                &std::env::var("API_KEY").expect("API key not defined"),
            );
        let response = client
            .delete_doc_project_role(DeleteDocProjectRoleRequest {
                project_id_or_name: "string".to_string(),
                user_id: "string".to_string(),
            });
        assert!(response.is_ok());
    }
    #[test]
    fn test_delete_guide_204_generated() {
        let client = Client::default()
            .with_api_key_auth(
                &std::env::var("API_KEY").expect("API key not defined"),
            )
            .with_cookie_auth(
                &std::env::var("API_KEY").expect("API key not defined"),
            );
        let response = client
            .delete_guide(DeleteGuideRequest {
                project_id_or_name: "string".to_string(),
                version_id: "string".to_string(),
                guide_id: "string".to_string(),
            });
        assert!(response.is_ok());
    }
    #[test]
    fn test_delete_guide_href_204_generated() {
        let client = Client::default()
            .with_api_key_auth(
                &std::env::var("API_KEY").expect("API key not defined"),
            )
            .with_cookie_auth(
                &std::env::var("API_KEY").expect("API key not defined"),
            );
        let response = client
            .delete_guide_href(DeleteGuideHrefRequest {
                project_id_or_name: "string".to_string(),
                version_id: "string".to_string(),
                guide_id: "string".to_string(),
                variant: GuideHrefVariantEnum::Prev,
            });
        assert!(response.is_ok());
    }
    #[test]
    fn test_delete_service_account_204_generated() {
        let client = Client::default()
            .with_api_key_auth(
                &std::env::var("API_KEY").expect("API key not defined"),
            )
            .with_cookie_auth(
                &std::env::var("API_KEY").expect("API key not defined"),
            );
        let response = client
            .delete_service_account(DeleteServiceAccountRequest {
                service_account_id: "3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a"
                    .to_string(),
            });
        assert!(response.is_ok());
    }
    #[test]
    fn test_health_check_200_generated() {
        let client = Client::default()
            .with_api_key_auth(
                &std::env::var("API_KEY").expect("API key not defined"),
            )
            .with_cookie_auth(
                &std::env::var("API_KEY").expect("API key not defined"),
            );
        let response = client.health_check();
        assert!(response.is_ok());
    }
    #[test]
    fn test_ping_check_200_generated() {
        let client = Client::default()
            .with_api_key_auth(
                &std::env::var("API_KEY").expect("API key not defined"),
            )
            .with_cookie_auth(
                &std::env::var("API_KEY").expect("API key not defined"),
            );
        let response = client.ping_check();
        assert!(response.is_ok());
    }
    #[test]
    fn test_list_api_links_200_generated() {
        let client = Client::default()
            .with_api_key_auth(
                &std::env::var("API_KEY").expect("API key not defined"),
            )
            .with_cookie_auth(
                &std::env::var("API_KEY").expect("API key not defined"),
            );
        let response = client
            .list_api_links(ListApiLinksRequest {
                doc_version_id: "3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a".to_string(),
            });
        assert!(response.is_ok());
    }
    #[test]
    fn test_get_api_link_200_generated() {
        let client = Client::default()
            .with_api_key_auth(
                &std::env::var("API_KEY").expect("API key not defined"),
            )
            .with_cookie_auth(
                &std::env::var("API_KEY").expect("API key not defined"),
            );
        let response = client
            .get_api_link(GetApiLinkRequest {
                link_id: "3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a".to_string(),
            });
        assert!(response.is_ok());
    }
    #[test]
    fn test_list_api_link_groups_200_generated() {
        let client = Client::default()
            .with_api_key_auth(
                &std::env::var("API_KEY").expect("API key not defined"),
            )
            .with_cookie_auth(
                &std::env::var("API_KEY").expect("API key not defined"),
            );
        let response = client
            .list_api_link_groups(ListApiLinkGroupsRequest {
                doc_version_id: "3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a".to_string(),
            });
        assert!(response.is_ok());
    }
    #[test]
    fn test_list_api_projects_200_generated() {
        let client = Client::default()
            .with_api_key_auth(
                &std::env::var("API_KEY").expect("API key not defined"),
            )
            .with_cookie_auth(
                &std::env::var("API_KEY").expect("API key not defined"),
            );
        let response = client.list_api_projects();
        assert!(response.is_ok());
    }
    #[test]
    fn test_get_api_project_200_generated() {
        let client = Client::default()
            .with_api_key_auth(
                &std::env::var("API_KEY").expect("API key not defined"),
            )
            .with_cookie_auth(
                &std::env::var("API_KEY").expect("API key not defined"),
            );
        let response = client
            .get_api_project(GetApiProjectRequest {
                project_id_or_name: "string".to_string(),
            });
        assert!(response.is_ok());
    }
    #[test]
    fn test_list_api_project_members_200_generated() {
        let client = Client::default()
            .with_api_key_auth(
                &std::env::var("API_KEY").expect("API key not defined"),
            )
            .with_cookie_auth(
                &std::env::var("API_KEY").expect("API key not defined"),
            );
        let response = client
            .list_api_project_members(ListApiProjectMembersRequest {
                project_id_or_name: "string".to_string(),
            });
        assert!(response.is_ok());
    }
    #[test]
    fn test_list_api_versions_200_generated() {
        let client = Client::default()
            .with_api_key_auth(
                &std::env::var("API_KEY").expect("API key not defined"),
            )
            .with_cookie_auth(
                &std::env::var("API_KEY").expect("API key not defined"),
            );
        let response = client
            .list_api_versions(ListApiVersionsRequest {
                project_id_or_name: "string".to_string(),
            });
        assert!(response.is_ok());
    }
    #[test]
    fn test_get_api_version_200_generated() {
        let client = Client::default()
            .with_api_key_auth(
                &std::env::var("API_KEY").expect("API key not defined"),
            )
            .with_cookie_auth(
                &std::env::var("API_KEY").expect("API key not defined"),
            );
        let response = client
            .get_api_version(GetApiVersionRequest {
                project_id_or_name: "string".to_string(),
                version_id_or_semver: "string".to_string(),
            });
        assert!(response.is_ok());
    }
    #[test]
    fn test_get_api_version_openapi_200_generated() {
        let client = Client::default()
            .with_api_key_auth(
                &std::env::var("API_KEY").expect("API key not defined"),
            )
            .with_cookie_auth(
                &std::env::var("API_KEY").expect("API key not defined"),
            );
        let response = client
            .get_api_version_openapi(GetApiVersionOpenapiRequest {
                project_id_or_name: "string".to_string(),
                version_id_or_semver: "string".to_string(),
            });
        assert!(response.is_ok());
    }
    #[test]
    fn test_get_api_version_stats_200_generated() {
        let client = Client::default()
            .with_api_key_auth(
                &std::env::var("API_KEY").expect("API key not defined"),
            )
            .with_cookie_auth(
                &std::env::var("API_KEY").expect("API key not defined"),
            );
        let response = client
            .get_api_version_stats(GetApiVersionStatsRequest {
                project_id_or_name: "string".to_string(),
                version_id_or_semver: "string".to_string(),
            });
        assert!(response.is_ok());
    }
    #[test]
    fn test_exchange_code_for_key_200_generated() {
        let client = Client::default()
            .with_api_key_auth(
                &std::env::var("API_KEY").expect("API key not defined"),
            )
            .with_cookie_auth(
                &std::env::var("API_KEY").expect("API key not defined"),
            );
        let response = client
            .exchange_code_for_key(ExchangeCodeForKeyRequest {
                code: "string".to_string(),
            });
        assert!(response.is_ok());
    }
    #[test]
    fn test_login_callback_303_generated() {
        let client = Client::default()
            .with_api_key_auth(
                &std::env::var("API_KEY").expect("API key not defined"),
            )
            .with_cookie_auth(
                &std::env::var("API_KEY").expect("API key not defined"),
            );
        let response = client
            .login_callback(LoginCallbackRequest {
                code: "string".to_string(),
                ..Default::default()
            });
        assert!(response.is_ok());
    }
    #[test]
    fn test_login_url_303_generated() {
        let client = Client::default()
            .with_api_key_auth(
                &std::env::var("API_KEY").expect("API key not defined"),
            )
            .with_cookie_auth(
                &std::env::var("API_KEY").expect("API key not defined"),
            );
        let response = client
            .login_url(LoginUrlRequest {
                ..Default::default()
            });
        assert!(response.is_ok());
    }
    #[test]
    fn test_cli_check_updates_200_generated() {
        let client = Client::default()
            .with_api_key_auth(
                &std::env::var("API_KEY").expect("API key not defined"),
            )
            .with_cookie_auth(
                &std::env::var("API_KEY").expect("API key not defined"),
            );
        let response = client
            .cli_check_updates(CliCheckUpdatesRequest {
                cli_version: "0.1.0".to_string(),
            });
        assert!(response.is_ok());
    }
    #[test]
    fn test_list_doc_projects_200_generated() {
        let client = Client::default()
            .with_api_key_auth(
                &std::env::var("API_KEY").expect("API key not defined"),
            )
            .with_cookie_auth(
                &std::env::var("API_KEY").expect("API key not defined"),
            );
        let response = client.list_doc_projects();
        assert!(response.is_ok());
    }
    #[test]
    fn test_get_doc_project_200_generated() {
        let client = Client::default()
            .with_api_key_auth(
                &std::env::var("API_KEY").expect("API key not defined"),
            )
            .with_cookie_auth(
                &std::env::var("API_KEY").expect("API key not defined"),
            );
        let response = client
            .get_doc_project(GetDocProjectRequest {
                project_id_or_name: "string".to_string(),
            });
        assert!(response.is_ok());
    }
    #[test]
    fn test_list_deployments_200_generated() {
        let client = Client::default()
            .with_api_key_auth(
                &std::env::var("API_KEY").expect("API key not defined"),
            )
            .with_cookie_auth(
                &std::env::var("API_KEY").expect("API key not defined"),
            );
        let response = client
            .list_deployments(ListDeploymentsRequest {
                project_id_or_name: "string".to_string(),
                ..Default::default()
            });
        assert!(response.is_ok());
    }
    #[test]
    fn test_get_deployment_200_generated() {
        let client = Client::default()
            .with_api_key_auth(
                &std::env::var("API_KEY").expect("API key not defined"),
            )
            .with_cookie_auth(
                &std::env::var("API_KEY").expect("API key not defined"),
            );
        let response = client
            .get_deployment(GetDeploymentRequest {
                project_id_or_name: "string".to_string(),
                deployment_id: "3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a".to_string(),
            });
        assert!(response.is_ok());
    }
    #[test]
    fn test_list_doc_project_members_200_generated() {
        let client = Client::default()
            .with_api_key_auth(
                &std::env::var("API_KEY").expect("API key not defined"),
            )
            .with_cookie_auth(
                &std::env::var("API_KEY").expect("API key not defined"),
            );
        let response = client
            .list_doc_project_members(ListDocProjectMembersRequest {
                project_id_or_name: "string".to_string(),
            });
        assert!(response.is_ok());
    }
    #[test]
    fn test_check_preview_200_generated() {
        let client = Client::default()
            .with_api_key_auth(
                &std::env::var("API_KEY").expect("API key not defined"),
            )
            .with_cookie_auth(
                &std::env::var("API_KEY").expect("API key not defined"),
            );
        let response = client
            .check_preview(CheckPreviewRequest {
                project_id_or_name: "string".to_string(),
            });
        assert!(response.is_ok());
    }
    #[test]
    fn test_get_doc_project_theme_200_generated() {
        let client = Client::default()
            .with_api_key_auth(
                &std::env::var("API_KEY").expect("API key not defined"),
            )
            .with_cookie_auth(
                &std::env::var("API_KEY").expect("API key not defined"),
            );
        let response = client
            .get_doc_project_theme(GetDocProjectThemeRequest {
                project_id_or_name: "string".to_string(),
            });
        assert!(response.is_ok());
    }
    #[test]
    fn test_list_doc_versions_200_generated() {
        let client = Client::default()
            .with_api_key_auth(
                &std::env::var("API_KEY").expect("API key not defined"),
            )
            .with_cookie_auth(
                &std::env::var("API_KEY").expect("API key not defined"),
            );
        let response = client
            .list_doc_versions(ListDocVersionsRequest {
                project_id_or_name: "string".to_string(),
            });
        assert!(response.is_ok());
    }
    #[test]
    fn test_get_doc_version_200_generated() {
        let client = Client::default()
            .with_api_key_auth(
                &std::env::var("API_KEY").expect("API key not defined"),
            )
            .with_cookie_auth(
                &std::env::var("API_KEY").expect("API key not defined"),
            );
        let response = client
            .get_doc_version(GetDocVersionRequest {
                project_id_or_name: "string".to_string(),
                version_id: "string".to_string(),
            });
        assert!(response.is_ok());
    }
    #[test]
    fn test_list_guides_200_generated() {
        let client = Client::default()
            .with_api_key_auth(
                &std::env::var("API_KEY").expect("API key not defined"),
            )
            .with_cookie_auth(
                &std::env::var("API_KEY").expect("API key not defined"),
            );
        let response = client
            .list_guides(ListGuidesRequest {
                project_id_or_name: "string".to_string(),
                version_id: "string".to_string(),
            });
        assert!(response.is_ok());
    }
    #[test]
    fn test_get_guide_200_generated() {
        let client = Client::default()
            .with_api_key_auth(
                &std::env::var("API_KEY").expect("API key not defined"),
            )
            .with_cookie_auth(
                &std::env::var("API_KEY").expect("API key not defined"),
            );
        let response = client
            .get_guide(GetGuideRequest {
                project_id_or_name: "string".to_string(),
                version_id: "string".to_string(),
                guide_id: "string".to_string(),
            });
        assert!(response.is_ok());
    }
    #[test]
    fn test_get_guide_content_200_generated() {
        let client = Client::default()
            .with_api_key_auth(
                &std::env::var("API_KEY").expect("API key not defined"),
            )
            .with_cookie_auth(
                &std::env::var("API_KEY").expect("API key not defined"),
            );
        let response = client
            .get_guide_content(GetGuideContentRequest {
                project_id_or_name: "string".to_string(),
                version_id: "string".to_string(),
                guide_id: "string".to_string(),
            });
        assert!(response.is_ok());
    }
    #[test]
    fn test_get_organization_200_generated() {
        let client = Client::default()
            .with_api_key_auth(
                &std::env::var("API_KEY").expect("API key not defined"),
            )
            .with_cookie_auth(
                &std::env::var("API_KEY").expect("API key not defined"),
            );
        let response = client.get_organization();
        assert!(response.is_ok());
    }
    #[test]
    fn test_get_assets_200_generated() {
        let client = Client::default()
            .with_api_key_auth(
                &std::env::var("API_KEY").expect("API key not defined"),
            )
            .with_cookie_auth(
                &std::env::var("API_KEY").expect("API key not defined"),
            );
        let response = client.get_assets();
        assert!(response.is_ok());
    }
    #[test]
    fn test_list_organization_members_200_generated() {
        let client = Client::default()
            .with_api_key_auth(
                &std::env::var("API_KEY").expect("API key not defined"),
            )
            .with_cookie_auth(
                &std::env::var("API_KEY").expect("API key not defined"),
            );
        let response = client.list_organization_members();
        assert!(response.is_ok());
    }
    #[test]
    fn test_get_organization_theme_200_generated() {
        let client = Client::default()
            .with_api_key_auth(
                &std::env::var("API_KEY").expect("API key not defined"),
            )
            .with_cookie_auth(
                &std::env::var("API_KEY").expect("API key not defined"),
            );
        let response = client.get_organization_theme();
        assert!(response.is_ok());
    }
    #[test]
    fn test_get_current_user_200_generated() {
        let client = Client::default()
            .with_api_key_auth(
                &std::env::var("API_KEY").expect("API key not defined"),
            )
            .with_cookie_auth(
                &std::env::var("API_KEY").expect("API key not defined"),
            );
        let response = client.get_current_user();
        assert!(response.is_ok());
    }
    #[test]
    fn test_get_api_key_200_generated() {
        let client = Client::default()
            .with_api_key_auth(
                &std::env::var("API_KEY").expect("API key not defined"),
            )
            .with_cookie_auth(
                &std::env::var("API_KEY").expect("API key not defined"),
            );
        let response = client.get_api_key();
        assert!(response.is_ok());
    }
    #[test]
    fn test_get_user_project_role_200_generated() {
        let client = Client::default()
            .with_api_key_auth(
                &std::env::var("API_KEY").expect("API key not defined"),
            )
            .with_cookie_auth(
                &std::env::var("API_KEY").expect("API key not defined"),
            );
        let response = client
            .get_user_project_role(GetUserProjectRoleRequest {
                project_type: ProjectTypeEnum::Api,
                ..Default::default()
            });
        assert!(response.is_ok());
    }
    #[test]
    fn test_get_service_accounts_200_generated() {
        let client = Client::default()
            .with_api_key_auth(
                &std::env::var("API_KEY").expect("API key not defined"),
            )
            .with_cookie_auth(
                &std::env::var("API_KEY").expect("API key not defined"),
            );
        let response = client.get_service_accounts();
        assert!(response.is_ok());
    }
    #[test]
    fn test_update_api_link_200_generated() {
        let client = Client::default()
            .with_api_key_auth(
                &std::env::var("API_KEY").expect("API key not defined"),
            )
            .with_cookie_auth(
                &std::env::var("API_KEY").expect("API key not defined"),
            );
        let response = client
            .update_api_link(UpdateApiLinkRequest {
                link_id: "3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a".to_string(),
                data: UpdateApiLink {
                    ..Default::default()
                },
            });
        assert!(response.is_ok());
    }
    #[test]
    fn test_update_api_link_group_200_generated() {
        let client = Client::default()
            .with_api_key_auth(
                &std::env::var("API_KEY").expect("API key not defined"),
            )
            .with_cookie_auth(
                &std::env::var("API_KEY").expect("API key not defined"),
            );
        let response = client
            .update_api_link_group(UpdateApiLinkGroupRequest {
                group_id: "3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a".to_string(),
                data: UpdateApiLinkGroup {
                    ..Default::default()
                },
            });
        assert!(response.is_ok());
    }
    #[test]
    fn test_update_api_project_200_generated() {
        let client = Client::default()
            .with_api_key_auth(
                &std::env::var("API_KEY").expect("API key not defined"),
            )
            .with_cookie_auth(
                &std::env::var("API_KEY").expect("API key not defined"),
            );
        let response = client
            .update_api_project(UpdateApiProjectRequest {
                project_id_or_name: "string".to_string(),
                data: UpdateApiProject {
                    ..Default::default()
                },
            });
        assert!(response.is_ok());
    }
    #[test]
    fn test_update_api_version_200_generated() {
        let client = Client::default()
            .with_api_key_auth(
                &std::env::var("API_KEY").expect("API key not defined"),
            )
            .with_cookie_auth(
                &std::env::var("API_KEY").expect("API key not defined"),
            );
        let response = client
            .update_api_version(UpdateApiVersionRequest {
                project_id_or_name: "string".to_string(),
                version_id_or_semver: "string".to_string(),
                data: UpdateApiVersion {
                    ..Default::default()
                },
            });
        assert!(response.is_ok());
    }
    #[test]
    fn test_update_doc_project_200_generated() {
        let client = Client::default()
            .with_api_key_auth(
                &std::env::var("API_KEY").expect("API key not defined"),
            )
            .with_cookie_auth(
                &std::env::var("API_KEY").expect("API key not defined"),
            );
        let response = client
            .update_doc_project(UpdateDocProjectRequest {
                project_id_or_name: "string".to_string(),
                data: UpdateDocProject {
                    ..Default::default()
                },
            });
        assert!(response.is_ok());
    }
    #[test]
    fn test_update_guide_200_generated() {
        let client = Client::default()
            .with_api_key_auth(
                &std::env::var("API_KEY").expect("API key not defined"),
            )
            .with_cookie_auth(
                &std::env::var("API_KEY").expect("API key not defined"),
            );
        let response = client
            .update_guide(UpdateGuideRequest {
                project_id_or_name: "string".to_string(),
                version_id: "string".to_string(),
                guide_id: "string".to_string(),
                data: UpdateGuide {
                    ..Default::default()
                },
            });
        assert!(response.is_ok());
    }
    #[test]
    fn test_create_api_link_201_generated() {
        let client = Client::default()
            .with_api_key_auth(
                &std::env::var("API_KEY").expect("API key not defined"),
            )
            .with_cookie_auth(
                &std::env::var("API_KEY").expect("API key not defined"),
            );
        let response = client
            .create_api_link(CreateApiLinkRequest {
                data: NewApiLink {
                    doc_version_id: "3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a"
                        .to_string(),
                    group_id: "3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a".to_string(),
                    nav_label: "string".to_string(),
                    policy: Union::LatestApiLinkPolicy(LatestApiLinkPolicy {
                        api_project_id: "3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a"
                            .to_string(),
                        type_field: LatestApiLinkPolicyTypeEnum::Latest,
                    }),
                    slug: "string".to_string(),
                    ..Default::default()
                },
            });
        assert!(response.is_ok());
    }
    #[test]
    fn test_reorder_api_links_200_generated() {
        let client = Client::default()
            .with_api_key_auth(
                &std::env::var("API_KEY").expect("API key not defined"),
            )
            .with_cookie_auth(
                &std::env::var("API_KEY").expect("API key not defined"),
            );
        let response = client
            .reorder_api_links(ReorderApiLinksRequest {
                data: ApiReorder {
                    doc_version_id: "3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a"
                        .to_string(),
                    groups: vec![
                        ApiLinkGroupReorder { id :
                        "3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a".to_string(), order :
                        123 }
                    ],
                    links: vec![
                        ApiLinkReorder { group_id :
                        "3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a".to_string(), id :
                        "3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a".to_string(), order :
                        123 }
                    ],
                },
            });
        assert!(response.is_ok());
    }
    #[test]
    fn test_create_api_link_group_201_generated() {
        let client = Client::default()
            .with_api_key_auth(
                &std::env::var("API_KEY").expect("API key not defined"),
            )
            .with_cookie_auth(
                &std::env::var("API_KEY").expect("API key not defined"),
            );
        let response = client
            .create_api_link_group(CreateApiLinkGroupRequest {
                data: NewApiLinkGroup {
                    doc_version_id: "3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a"
                        .to_string(),
                    nav_label: "string".to_string(),
                    slug: "string".to_string(),
                },
            });
        assert!(response.is_ok());
    }
    #[test]
    fn test_create_api_project_201_generated() {
        let client = Client::default()
            .with_api_key_auth(
                &std::env::var("API_KEY").expect("API key not defined"),
            )
            .with_cookie_auth(
                &std::env::var("API_KEY").expect("API key not defined"),
            );
        let response = client
            .create_api_project(CreateApiProjectRequest {
                data: NewApiProject {
                    title: "my-api-spec".to_string(),
                },
            });
        assert!(response.is_ok());
    }
    #[test]
    fn test_grant_api_project_role_201_generated() {
        let client = Client::default()
            .with_api_key_auth(
                &std::env::var("API_KEY").expect("API key not defined"),
            )
            .with_cookie_auth(
                &std::env::var("API_KEY").expect("API key not defined"),
            );
        let response = client
            .grant_api_project_role(GrantApiProjectRoleRequest {
                project_id_or_name: "string".to_string(),
                data: NewProjectRole {
                    role: ProjectRoleEnum::Viewer,
                    user_id: "3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a".to_string(),
                },
            });
        assert!(response.is_ok());
    }
    #[test]
    fn test_create_api_version_201_generated() {
        let client = Client::default()
            .with_api_key_auth(
                &std::env::var("API_KEY").expect("API key not defined"),
            )
            .with_cookie_auth(
                &std::env::var("API_KEY").expect("API key not defined"),
            );
        let response = client
            .create_api_version(CreateApiVersionRequest {
                project_id_or_name: "string".to_string(),
                data: NewApiVersion {
                    openapi: "string".to_string(),
                    semver: "string".to_string(),
                    ..Default::default()
                },
            });
        assert!(response.is_ok());
    }
    #[test]
    fn test_create_doc_project_201_generated() {
        let client = Client::default()
            .with_api_key_auth(
                &std::env::var("API_KEY").expect("API key not defined"),
            )
            .with_cookie_auth(
                &std::env::var("API_KEY").expect("API key not defined"),
            );
        let response = client
            .create_doc_project(CreateDocProjectRequest {
                data: NewDocProject {
                    title: "my-company-docs".to_string(),
                },
            });
        assert!(response.is_ok());
    }
    #[test]
    fn test_trigger_deployment_201_generated() {
        let client = Client::default()
            .with_api_key_auth(
                &std::env::var("API_KEY").expect("API key not defined"),
            )
            .with_cookie_auth(
                &std::env::var("API_KEY").expect("API key not defined"),
            );
        let response = client
            .trigger_deployment(TriggerDeploymentRequest {
                project_id_or_name: "string".to_string(),
                data: NewDeployment {
                    doc_version_id: "3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a"
                        .to_string(),
                    target: DeploymentTargetEnum::Production,
                },
            });
        assert!(response.is_ok());
    }
    #[test]
    fn test_grant_doc_project_role_201_generated() {
        let client = Client::default()
            .with_api_key_auth(
                &std::env::var("API_KEY").expect("API key not defined"),
            )
            .with_cookie_auth(
                &std::env::var("API_KEY").expect("API key not defined"),
            );
        let response = client
            .grant_doc_project_role(GrantDocProjectRoleRequest {
                project_id_or_name: "string".to_string(),
                data: NewProjectRole {
                    role: ProjectRoleEnum::Viewer,
                    user_id: "3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a".to_string(),
                },
            });
        assert!(response.is_ok());
    }
    #[test]
    fn test_create_guide_201_generated() {
        let client = Client::default()
            .with_api_key_auth(
                &std::env::var("API_KEY").expect("API key not defined"),
            )
            .with_cookie_auth(
                &std::env::var("API_KEY").expect("API key not defined"),
            );
        let response = client
            .create_guide(CreateGuideRequest {
                project_id_or_name: "string".to_string(),
                version_id: "string".to_string(),
                data: NewGuide {
                    content: "string".to_string(),
                    is_parent: true,
                    nav_label: "string".to_string(),
                    slug: "string".to_string(),
                    ..Default::default()
                },
            });
        assert!(response.is_ok());
    }
    #[test]
    fn test_reorder_guides_200_generated() {
        let client = Client::default()
            .with_api_key_auth(
                &std::env::var("API_KEY").expect("API key not defined"),
            )
            .with_cookie_auth(
                &std::env::var("API_KEY").expect("API key not defined"),
            );
        let response = client
            .reorder_guides(ReorderGuidesRequest {
                project_id_or_name: "string".to_string(),
                version_id: "string".to_string(),
                data: vec![
                    ReorderGuide { id : "3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a"
                    .to_string(), order : 123, ..Default::default() }
                ],
            });
        assert!(response.is_ok());
    }
    #[test]
    fn test_create_organization_201_generated() {
        let client = Client::default()
            .with_api_key_auth(
                &std::env::var("API_KEY").expect("API key not defined"),
            )
            .with_cookie_auth(
                &std::env::var("API_KEY").expect("API key not defined"),
            );
        let response = client
            .create_organization(CreateOrganizationRequest {
                data: NewOrganization {
                    name: "string".to_string(),
                    subdomain: "string".to_string(),
                },
            });
        assert!(response.is_ok());
    }
    #[test]
    fn test_upload_assets_200_generated() {
        let client = Client::default()
            .with_api_key_auth(
                &std::env::var("API_KEY").expect("API key not defined"),
            )
            .with_cookie_auth(
                &std::env::var("API_KEY").expect("API key not defined"),
            );
        let response = client
            .upload_assets(UploadAssetsRequest {
                data: AssetUpload {
                    file: "path/to/file.pdf".to_string(),
                },
            });
        assert!(response.is_ok());
    }
    #[test]
    fn test_create_sdk_201_generated() {
        let client = Client::default()
            .with_api_key_auth(
                &std::env::var("API_KEY").expect("API key not defined"),
            )
            .with_cookie_auth(
                &std::env::var("API_KEY").expect("API key not defined"),
            );
        let response = client
            .create_sdk(CreateSdkRequest {
                data: SdkProject {
                    api_project_version_id: "3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a"
                        .to_string(),
                    language: GenerationLanguageEnum::Python,
                    semver: "1.0.0".to_string(),
                    ..Default::default()
                },
            });
        assert!(response.is_ok());
    }
    #[test]
    fn test_update_sdk_200_generated() {
        let client = Client::default()
            .with_api_key_auth(
                &std::env::var("API_KEY").expect("API key not defined"),
            )
            .with_cookie_auth(
                &std::env::var("API_KEY").expect("API key not defined"),
            );
        let response = client
            .update_sdk(UpdateSdkRequest {
                data: UpdateSdkProject {
                    api_project_version_id: "3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a"
                        .to_string(),
                    language: GenerationLanguageEnum::Python,
                    semver: "1.0.0".to_string(),
                },
            });
        assert!(response.is_ok());
    }
    #[test]
    fn test_stateless_generate_sdk_201_generated() {
        let client = Client::default()
            .with_api_key_auth(
                &std::env::var("API_KEY").expect("API key not defined"),
            )
            .with_cookie_auth(
                &std::env::var("API_KEY").expect("API key not defined"),
            );
        let response = client
            .stateless_generate_sdk(StatelessGenerateSdkRequest {
                data: StatelessGenerateSdk {
                    language: GenerationLanguageEnum::Python,
                    openapi: "openapi: 3.0.0".to_string(),
                    ..Default::default()
                },
            });
        assert!(response.is_ok());
    }
    #[test]
    fn test_invite_user_201_generated() {
        let client = Client::default()
            .with_api_key_auth(
                &std::env::var("API_KEY").expect("API key not defined"),
            )
            .with_cookie_auth(
                &std::env::var("API_KEY").expect("API key not defined"),
            );
        let response = client
            .invite_user(InviteUserRequest {
                data: Invite {
                    email: "user@example.com".to_string(),
                    role: OrganizationRoleEnum::Admin,
                },
            });
        assert!(response.is_ok());
    }
    #[test]
    fn test_create_service_account_200_generated() {
        let client = Client::default()
            .with_api_key_auth(
                &std::env::var("API_KEY").expect("API key not defined"),
            )
            .with_cookie_auth(
                &std::env::var("API_KEY").expect("API key not defined"),
            );
        let response = client
            .create_service_account(CreateServiceAccountRequest {
                data: CreateServiceAccount {
                    name: "Documentation Publisher Service Account".to_string(),
                    project_roles: vec![
                        UserProjectRole { project_id_or_name : "string".to_string(),
                        project_type : ProjectTypeEnum::Api, role :
                        ProjectRoleEnum::Viewer }
                    ],
                },
            });
        assert!(response.is_ok());
    }
    #[test]
    fn test_vercel_webhook_202_generated() {
        let client = Client::default()
            .with_api_key_auth(
                &std::env::var("API_KEY").expect("API key not defined"),
            )
            .with_cookie_auth(
                &std::env::var("API_KEY").expect("API key not defined"),
            );
        let response = client
            .vercel_webhook(VercelWebhookRequest {
                data: serde_json::json!({}),
            });
        assert!(response.is_ok());
    }
    #[test]
    fn test_update_doc_project_theme_200_generated() {
        let client = Client::default()
            .with_api_key_auth(
                &std::env::var("API_KEY").expect("API key not defined"),
            )
            .with_cookie_auth(
                &std::env::var("API_KEY").expect("API key not defined"),
            );
        let response = client
            .update_doc_project_theme(UpdateDocProjectThemeRequest {
                project_id_or_name: "string".to_string(),
                data: ThemeValues {
                    ..Default::default()
                },
            });
        assert!(response.is_ok());
    }
    #[test]
    fn test_update_organization_theme_200_generated() {
        let client = Client::default()
            .with_api_key_auth(
                &std::env::var("API_KEY").expect("API key not defined"),
            )
            .with_cookie_auth(
                &std::env::var("API_KEY").expect("API key not defined"),
            );
        let response = client
            .update_organization_theme(UpdateOrganizationThemeRequest {
                data: ThemeValues {
                    ..Default::default()
                },
            });
        assert!(response.is_ok());
    }
}