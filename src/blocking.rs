/// Generatedby Sideko (sideko.dev)
use crate::auth;
use crate::error_enums;
use crate::request_types::*;
use crate::result;
use crate::schemas::*;
#[allow(unused)]
use reqwest::blocking::multipart as reqwest_multipart;
use reqwest::blocking::Client as ReqwestClient;
use reqwest::blocking::RequestBuilder as ReqwestRequestBuilder;
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
        self.auth.insert(
            "ApiKeyAuth".to_string(),
            auth::AuthProvider::KeyHeader("x-sideko-key".to_string(), val.to_string()),
        );
        self
    }
    /// Authentication  builder function to store api-key credentials in the client
    pub fn with_cookie_auth(mut self, val: &str) -> Self {
        self.auth.insert(
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
        let unauthed_builder = ReqwestClient::default().delete(&url).query(&query_params);
        let authed_builder =
            self.builder_with_auth(unauthed_builder, &["ApiKeyAuth", "CookieAuth"]);
        let response = authed_builder.send().map_err(result::Error::Dispatch)?;
        let status_code = response.status().as_u16();
        match status_code {
            204 => {
                let response_text = response.text().unwrap_or_default();
                if response_text.is_empty() {
                    return Ok(serde_json::json!({}));
                }
                let data = serde_json::from_str::<serde_json::Value>(&response_text).unwrap();
                Ok(data)
            }
            400 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "DELETE".to_string(),
                    url: url.to_string(),
                    data: error_enums::DeleteApiLinkErrors::Status400(data),
                })
            }
            401 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "DELETE".to_string(),
                    url: url.to_string(),
                    data: error_enums::DeleteApiLinkErrors::Status401(data),
                })
            }
            404 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "DELETE".to_string(),
                    url: url.to_string(),
                    data: error_enums::DeleteApiLinkErrors::Status404(data),
                })
            }
            500 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "DELETE".to_string(),
                    url: url.to_string(),
                    data: error_enums::DeleteApiLinkErrors::Status500(data),
                })
            }
            _ => Err(result::Error::BlockingApiError {
                status_code,
                method: "".to_string(),
                url: url.to_string(),
                response,
            }),
        }
    }
    pub fn delete_api_link_group(
        &self,
        request: DeleteApiLinkGroupRequest,
    ) -> result::Result<serde_json::Value, error_enums::DeleteApiLinkGroupErrors> {
        let endpoint = format!("/api_link_group/{}", request.group_id);
        let url = format!("{}{}", self.base_url, endpoint);
        let query_params: Vec<(&str, String)> = vec![];
        let unauthed_builder = ReqwestClient::default().delete(&url).query(&query_params);
        let authed_builder =
            self.builder_with_auth(unauthed_builder, &["ApiKeyAuth", "CookieAuth"]);
        let response = authed_builder.send().map_err(result::Error::Dispatch)?;
        let status_code = response.status().as_u16();
        match status_code {
            204 => {
                let response_text = response.text().unwrap_or_default();
                if response_text.is_empty() {
                    return Ok(serde_json::json!({}));
                }
                let data = serde_json::from_str::<serde_json::Value>(&response_text).unwrap();
                Ok(data)
            }
            400 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "DELETE".to_string(),
                    url: url.to_string(),
                    data: error_enums::DeleteApiLinkGroupErrors::Status400(data),
                })
            }
            401 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "DELETE".to_string(),
                    url: url.to_string(),
                    data: error_enums::DeleteApiLinkGroupErrors::Status401(data),
                })
            }
            404 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "DELETE".to_string(),
                    url: url.to_string(),
                    data: error_enums::DeleteApiLinkGroupErrors::Status404(data),
                })
            }
            500 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "DELETE".to_string(),
                    url: url.to_string(),
                    data: error_enums::DeleteApiLinkGroupErrors::Status500(data),
                })
            }
            _ => Err(result::Error::BlockingApiError {
                status_code,
                method: "".to_string(),
                url: url.to_string(),
                response,
            }),
        }
    }
    pub fn delete_api_project(
        &self,
        request: DeleteApiProjectRequest,
    ) -> result::Result<serde_json::Value, error_enums::DeleteApiProjectErrors> {
        let endpoint = format!("/api_project/{}", request.project_id_or_name);
        let url = format!("{}{}", self.base_url, endpoint);
        let query_params: Vec<(&str, String)> = vec![];
        let unauthed_builder = ReqwestClient::default().delete(&url).query(&query_params);
        let authed_builder =
            self.builder_with_auth(unauthed_builder, &["ApiKeyAuth", "CookieAuth"]);
        let response = authed_builder.send().map_err(result::Error::Dispatch)?;
        let status_code = response.status().as_u16();
        match status_code {
            204 => {
                let response_text = response.text().unwrap_or_default();
                if response_text.is_empty() {
                    return Ok(serde_json::json!({}));
                }
                let data = serde_json::from_str::<serde_json::Value>(&response_text).unwrap();
                Ok(data)
            }
            400 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "DELETE".to_string(),
                    url: url.to_string(),
                    data: error_enums::DeleteApiProjectErrors::Status400(data),
                })
            }
            401 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "DELETE".to_string(),
                    url: url.to_string(),
                    data: error_enums::DeleteApiProjectErrors::Status401(data),
                })
            }
            404 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "DELETE".to_string(),
                    url: url.to_string(),
                    data: error_enums::DeleteApiProjectErrors::Status404(data),
                })
            }
            500 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "DELETE".to_string(),
                    url: url.to_string(),
                    data: error_enums::DeleteApiProjectErrors::Status500(data),
                })
            }
            _ => Err(result::Error::BlockingApiError {
                status_code,
                method: "".to_string(),
                url: url.to_string(),
                response,
            }),
        }
    }
    pub fn delete_api_project_role(
        &self,
        request: DeleteApiProjectRoleRequest,
    ) -> result::Result<serde_json::Value, error_enums::DeleteApiProjectRoleErrors> {
        let endpoint = format!(
            "/api_project/{}/role/{}",
            request.project_id_or_name, request.user_id
        );
        let url = format!("{}{}", self.base_url, endpoint);
        let query_params: Vec<(&str, String)> = vec![];
        let unauthed_builder = ReqwestClient::default().delete(&url).query(&query_params);
        let authed_builder =
            self.builder_with_auth(unauthed_builder, &["ApiKeyAuth", "CookieAuth"]);
        let response = authed_builder.send().map_err(result::Error::Dispatch)?;
        let status_code = response.status().as_u16();
        match status_code {
            204 => {
                let response_text = response.text().unwrap_or_default();
                if response_text.is_empty() {
                    return Ok(serde_json::json!({}));
                }
                let data = serde_json::from_str::<serde_json::Value>(&response_text).unwrap();
                Ok(data)
            }
            400 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "DELETE".to_string(),
                    url: url.to_string(),
                    data: error_enums::DeleteApiProjectRoleErrors::Status400(data),
                })
            }
            401 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "DELETE".to_string(),
                    url: url.to_string(),
                    data: error_enums::DeleteApiProjectRoleErrors::Status401(data),
                })
            }
            404 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "DELETE".to_string(),
                    url: url.to_string(),
                    data: error_enums::DeleteApiProjectRoleErrors::Status404(data),
                })
            }
            500 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "DELETE".to_string(),
                    url: url.to_string(),
                    data: error_enums::DeleteApiProjectRoleErrors::Status500(data),
                })
            }
            _ => Err(result::Error::BlockingApiError {
                status_code,
                method: "".to_string(),
                url: url.to_string(),
                response,
            }),
        }
    }
    pub fn delete_doc_project(
        &self,
        request: DeleteDocProjectRequest,
    ) -> result::Result<serde_json::Value, error_enums::DeleteDocProjectErrors> {
        let endpoint = format!("/doc_project/{}", request.project_id_or_name);
        let url = format!("{}{}", self.base_url, endpoint);
        let query_params: Vec<(&str, String)> = vec![];
        let unauthed_builder = ReqwestClient::default().delete(&url).query(&query_params);
        let authed_builder =
            self.builder_with_auth(unauthed_builder, &["ApiKeyAuth", "CookieAuth"]);
        let response = authed_builder.send().map_err(result::Error::Dispatch)?;
        let status_code = response.status().as_u16();
        match status_code {
            204 => {
                let response_text = response.text().unwrap_or_default();
                if response_text.is_empty() {
                    return Ok(serde_json::json!({}));
                }
                let data = serde_json::from_str::<serde_json::Value>(&response_text).unwrap();
                Ok(data)
            }
            _ => Err(result::Error::BlockingApiError {
                status_code,
                method: "".to_string(),
                url: url.to_string(),
                response,
            }),
        }
    }
    pub fn delete_doc_project_role(
        &self,
        request: DeleteDocProjectRoleRequest,
    ) -> result::Result<serde_json::Value, error_enums::DeleteDocProjectRoleErrors> {
        let endpoint = format!(
            "/doc_project/{}/role/{}",
            request.project_id_or_name, request.user_id
        );
        let url = format!("{}{}", self.base_url, endpoint);
        let query_params: Vec<(&str, String)> = vec![];
        let unauthed_builder = ReqwestClient::default().delete(&url).query(&query_params);
        let authed_builder =
            self.builder_with_auth(unauthed_builder, &["ApiKeyAuth", "CookieAuth"]);
        let response = authed_builder.send().map_err(result::Error::Dispatch)?;
        let status_code = response.status().as_u16();
        match status_code {
            204 => {
                let response_text = response.text().unwrap_or_default();
                if response_text.is_empty() {
                    return Ok(serde_json::json!({}));
                }
                let data = serde_json::from_str::<serde_json::Value>(&response_text).unwrap();
                Ok(data)
            }
            400 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "DELETE".to_string(),
                    url: url.to_string(),
                    data: error_enums::DeleteDocProjectRoleErrors::Status400(data),
                })
            }
            401 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "DELETE".to_string(),
                    url: url.to_string(),
                    data: error_enums::DeleteDocProjectRoleErrors::Status401(data),
                })
            }
            404 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "DELETE".to_string(),
                    url: url.to_string(),
                    data: error_enums::DeleteDocProjectRoleErrors::Status404(data),
                })
            }
            500 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "DELETE".to_string(),
                    url: url.to_string(),
                    data: error_enums::DeleteDocProjectRoleErrors::Status500(data),
                })
            }
            _ => Err(result::Error::BlockingApiError {
                status_code,
                method: "".to_string(),
                url: url.to_string(),
                response,
            }),
        }
    }
    pub fn delete_guide(
        &self,
        request: DeleteGuideRequest,
    ) -> result::Result<serde_json::Value, error_enums::DeleteGuideErrors> {
        let endpoint = format!(
            "/doc_project/{}/version/{}/guide/{}",
            request.project_id_or_name, request.version_id, request.guide_id
        );
        let url = format!("{}{}", self.base_url, endpoint);
        let query_params: Vec<(&str, String)> = vec![];
        let unauthed_builder = ReqwestClient::default().delete(&url).query(&query_params);
        let authed_builder =
            self.builder_with_auth(unauthed_builder, &["ApiKeyAuth", "CookieAuth"]);
        let response = authed_builder.send().map_err(result::Error::Dispatch)?;
        let status_code = response.status().as_u16();
        match status_code {
            204 => {
                let response_text = response.text().unwrap_or_default();
                if response_text.is_empty() {
                    return Ok(serde_json::json!({}));
                }
                let data = serde_json::from_str::<serde_json::Value>(&response_text).unwrap();
                Ok(data)
            }
            400 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "DELETE".to_string(),
                    url: url.to_string(),
                    data: error_enums::DeleteGuideErrors::Status400(data),
                })
            }
            401 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "DELETE".to_string(),
                    url: url.to_string(),
                    data: error_enums::DeleteGuideErrors::Status401(data),
                })
            }
            404 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "DELETE".to_string(),
                    url: url.to_string(),
                    data: error_enums::DeleteGuideErrors::Status404(data),
                })
            }
            500 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "DELETE".to_string(),
                    url: url.to_string(),
                    data: error_enums::DeleteGuideErrors::Status500(data),
                })
            }
            _ => Err(result::Error::BlockingApiError {
                status_code,
                method: "".to_string(),
                url: url.to_string(),
                response,
            }),
        }
    }
    pub fn delete_guide_href(
        &self,
        request: DeleteGuideHrefRequest,
    ) -> result::Result<serde_json::Value, error_enums::DeleteGuideHrefErrors> {
        let endpoint = format!(
            "/doc_project/{}/version/{}/guide/{}/href",
            request.project_id_or_name, request.version_id, request.guide_id
        );
        let url = format!("{}{}", self.base_url, endpoint);
        let mut query_params: Vec<(&str, String)> = vec![];
        query_params.push(("variant", request.variant.to_string()));
        let unauthed_builder = ReqwestClient::default().delete(&url).query(&query_params);
        let authed_builder =
            self.builder_with_auth(unauthed_builder, &["ApiKeyAuth", "CookieAuth"]);
        let response = authed_builder.send().map_err(result::Error::Dispatch)?;
        let status_code = response.status().as_u16();
        match status_code {
            204 => {
                let response_text = response.text().unwrap_or_default();
                if response_text.is_empty() {
                    return Ok(serde_json::json!({}));
                }
                let data = serde_json::from_str::<serde_json::Value>(&response_text).unwrap();
                Ok(data)
            }
            400 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "DELETE".to_string(),
                    url: url.to_string(),
                    data: error_enums::DeleteGuideHrefErrors::Status400(data),
                })
            }
            401 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "DELETE".to_string(),
                    url: url.to_string(),
                    data: error_enums::DeleteGuideHrefErrors::Status401(data),
                })
            }
            404 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "DELETE".to_string(),
                    url: url.to_string(),
                    data: error_enums::DeleteGuideHrefErrors::Status404(data),
                })
            }
            500 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "DELETE".to_string(),
                    url: url.to_string(),
                    data: error_enums::DeleteGuideHrefErrors::Status500(data),
                })
            }
            _ => Err(result::Error::BlockingApiError {
                status_code,
                method: "".to_string(),
                url: url.to_string(),
                response,
            }),
        }
    }
    /// Delete an asset in your organization
    pub fn delete_asset(
        &self,
        request: DeleteAssetRequest,
    ) -> result::Result<serde_json::Value, error_enums::DeleteAssetErrors> {
        let endpoint = format!("/organization/asset/{}", request.asset_id);
        let url = format!("{}{}", self.base_url, endpoint);
        let query_params: Vec<(&str, String)> = vec![];
        let unauthed_builder = ReqwestClient::default().delete(&url).query(&query_params);
        let authed_builder =
            self.builder_with_auth(unauthed_builder, &["ApiKeyAuth", "CookieAuth"]);
        let response = authed_builder.send().map_err(result::Error::Dispatch)?;
        let status_code = response.status().as_u16();
        match status_code {
            204 => {
                let response_text = response.text().unwrap_or_default();
                if response_text.is_empty() {
                    return Ok(serde_json::json!({}));
                }
                let data = serde_json::from_str::<serde_json::Value>(&response_text).unwrap();
                Ok(data)
            }
            400 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "DELETE".to_string(),
                    url: url.to_string(),
                    data: error_enums::DeleteAssetErrors::Status400(data),
                })
            }
            401 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "DELETE".to_string(),
                    url: url.to_string(),
                    data: error_enums::DeleteAssetErrors::Status401(data),
                })
            }
            404 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "DELETE".to_string(),
                    url: url.to_string(),
                    data: error_enums::DeleteAssetErrors::Status404(data),
                })
            }
            500 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "DELETE".to_string(),
                    url: url.to_string(),
                    data: error_enums::DeleteAssetErrors::Status500(data),
                })
            }
            _ => Err(result::Error::BlockingApiError {
                status_code,
                method: "".to_string(),
                url: url.to_string(),
                response,
            }),
        }
    }
    pub fn delete_service_account(
        &self,
        request: DeleteServiceAccountRequest,
    ) -> result::Result<serde_json::Value, error_enums::DeleteServiceAccountErrors> {
        let endpoint = format!("/user/service_account/{}", request.service_account_id);
        let url = format!("{}{}", self.base_url, endpoint);
        let query_params: Vec<(&str, String)> = vec![];
        let unauthed_builder = ReqwestClient::default().delete(&url).query(&query_params);
        let authed_builder =
            self.builder_with_auth(unauthed_builder, &["ApiKeyAuth", "CookieAuth"]);
        let response = authed_builder.send().map_err(result::Error::Dispatch)?;
        let status_code = response.status().as_u16();
        match status_code {
            204 => {
                let response_text = response.text().unwrap_or_default();
                if response_text.is_empty() {
                    return Ok(serde_json::json!({}));
                }
                let data = serde_json::from_str::<serde_json::Value>(&response_text).unwrap();
                Ok(data)
            }
            400 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "DELETE".to_string(),
                    url: url.to_string(),
                    data: error_enums::DeleteServiceAccountErrors::Status400(data),
                })
            }
            401 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "DELETE".to_string(),
                    url: url.to_string(),
                    data: error_enums::DeleteServiceAccountErrors::Status401(data),
                })
            }
            404 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "DELETE".to_string(),
                    url: url.to_string(),
                    data: error_enums::DeleteServiceAccountErrors::Status404(data),
                })
            }
            500 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "DELETE".to_string(),
                    url: url.to_string(),
                    data: error_enums::DeleteServiceAccountErrors::Status500(data),
                })
            }
            _ => Err(result::Error::BlockingApiError {
                status_code,
                method: "".to_string(),
                url: url.to_string(),
                response,
            }),
        }
    }
    pub fn health_check(
        &self,
    ) -> result::Result<GetHealthResponse, error_enums::HealthCheckErrors> {
        let endpoint = "/_health";
        let url = format!("{}{}", self.base_url, endpoint);
        let query_params: Vec<(&str, String)> = vec![];
        let unauthed_builder = ReqwestClient::default().get(&url).query(&query_params);
        let authed_builder =
            self.builder_with_auth(unauthed_builder, &["ApiKeyAuth", "CookieAuth"]);
        let response = authed_builder.send().map_err(result::Error::Dispatch)?;
        let status_code = response.status().as_u16();
        match status_code {
            200 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<GetHealthResponse>(&response_text).unwrap();
                Ok(data)
            }
            _ => Err(result::Error::BlockingApiError {
                status_code,
                method: "".to_string(),
                url: url.to_string(),
                response,
            }),
        }
    }
    pub fn ping_check(&self) -> result::Result<GetPingResponse, error_enums::PingCheckErrors> {
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
                let data = serde_json::from_str::<GetPingResponse>(&response_text).unwrap();
                Ok(data)
            }
            _ => Err(result::Error::BlockingApiError {
                status_code,
                method: "".to_string(),
                url: url.to_string(),
                response,
            }),
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
        let authed_builder =
            self.builder_with_auth(unauthed_builder, &["ApiKeyAuth", "CookieAuth"]);
        let response = authed_builder.send().map_err(result::Error::Dispatch)?;
        let status_code = response.status().as_u16();
        match status_code {
            200 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Vec<ApiLink>>(&response_text).unwrap();
                Ok(data)
            }
            400 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "GET".to_string(),
                    url: url.to_string(),
                    data: error_enums::ListApiLinksErrors::Status400(data),
                })
            }
            401 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "GET".to_string(),
                    url: url.to_string(),
                    data: error_enums::ListApiLinksErrors::Status401(data),
                })
            }
            404 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "GET".to_string(),
                    url: url.to_string(),
                    data: error_enums::ListApiLinksErrors::Status404(data),
                })
            }
            500 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "GET".to_string(),
                    url: url.to_string(),
                    data: error_enums::ListApiLinksErrors::Status500(data),
                })
            }
            _ => Err(result::Error::BlockingApiError {
                status_code,
                method: "".to_string(),
                url: url.to_string(),
                response,
            }),
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
        let authed_builder =
            self.builder_with_auth(unauthed_builder, &["ApiKeyAuth", "CookieAuth"]);
        let response = authed_builder.send().map_err(result::Error::Dispatch)?;
        let status_code = response.status().as_u16();
        match status_code {
            200 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<ApiLink>(&response_text).unwrap();
                Ok(data)
            }
            400 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "GET".to_string(),
                    url: url.to_string(),
                    data: error_enums::GetApiLinkErrors::Status400(data),
                })
            }
            401 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "GET".to_string(),
                    url: url.to_string(),
                    data: error_enums::GetApiLinkErrors::Status401(data),
                })
            }
            404 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "GET".to_string(),
                    url: url.to_string(),
                    data: error_enums::GetApiLinkErrors::Status404(data),
                })
            }
            500 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "GET".to_string(),
                    url: url.to_string(),
                    data: error_enums::GetApiLinkErrors::Status500(data),
                })
            }
            _ => Err(result::Error::BlockingApiError {
                status_code,
                method: "".to_string(),
                url: url.to_string(),
                response,
            }),
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
        let authed_builder =
            self.builder_with_auth(unauthed_builder, &["ApiKeyAuth", "CookieAuth"]);
        let response = authed_builder.send().map_err(result::Error::Dispatch)?;
        let status_code = response.status().as_u16();
        match status_code {
            200 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Vec<ApiLinkGroup>>(&response_text).unwrap();
                Ok(data)
            }
            400 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "GET".to_string(),
                    url: url.to_string(),
                    data: error_enums::ListApiLinkGroupsErrors::Status400(data),
                })
            }
            401 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "GET".to_string(),
                    url: url.to_string(),
                    data: error_enums::ListApiLinkGroupsErrors::Status401(data),
                })
            }
            404 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "GET".to_string(),
                    url: url.to_string(),
                    data: error_enums::ListApiLinkGroupsErrors::Status404(data),
                })
            }
            500 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "GET".to_string(),
                    url: url.to_string(),
                    data: error_enums::ListApiLinkGroupsErrors::Status500(data),
                })
            }
            _ => Err(result::Error::BlockingApiError {
                status_code,
                method: "".to_string(),
                url: url.to_string(),
                response,
            }),
        }
    }
    pub fn list_api_projects(
        &self,
    ) -> result::Result<Vec<ApiProject>, error_enums::ListApiProjectsErrors> {
        let endpoint = "/api_project";
        let url = format!("{}{}", self.base_url, endpoint);
        let query_params: Vec<(&str, String)> = vec![];
        let unauthed_builder = ReqwestClient::default().get(&url).query(&query_params);
        let authed_builder =
            self.builder_with_auth(unauthed_builder, &["ApiKeyAuth", "CookieAuth"]);
        let response = authed_builder.send().map_err(result::Error::Dispatch)?;
        let status_code = response.status().as_u16();
        match status_code {
            200 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Vec<ApiProject>>(&response_text).unwrap();
                Ok(data)
            }
            400 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "GET".to_string(),
                    url: url.to_string(),
                    data: error_enums::ListApiProjectsErrors::Status400(data),
                })
            }
            401 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "GET".to_string(),
                    url: url.to_string(),
                    data: error_enums::ListApiProjectsErrors::Status401(data),
                })
            }
            404 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "GET".to_string(),
                    url: url.to_string(),
                    data: error_enums::ListApiProjectsErrors::Status404(data),
                })
            }
            500 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "GET".to_string(),
                    url: url.to_string(),
                    data: error_enums::ListApiProjectsErrors::Status500(data),
                })
            }
            _ => Err(result::Error::BlockingApiError {
                status_code,
                method: "".to_string(),
                url: url.to_string(),
                response,
            }),
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
        let authed_builder =
            self.builder_with_auth(unauthed_builder, &["ApiKeyAuth", "CookieAuth"]);
        let response = authed_builder.send().map_err(result::Error::Dispatch)?;
        let status_code = response.status().as_u16();
        match status_code {
            200 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<ApiProject>(&response_text).unwrap();
                Ok(data)
            }
            400 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "GET".to_string(),
                    url: url.to_string(),
                    data: error_enums::GetApiProjectErrors::Status400(data),
                })
            }
            401 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "GET".to_string(),
                    url: url.to_string(),
                    data: error_enums::GetApiProjectErrors::Status401(data),
                })
            }
            404 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "GET".to_string(),
                    url: url.to_string(),
                    data: error_enums::GetApiProjectErrors::Status404(data),
                })
            }
            500 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "GET".to_string(),
                    url: url.to_string(),
                    data: error_enums::GetApiProjectErrors::Status500(data),
                })
            }
            _ => Err(result::Error::BlockingApiError {
                status_code,
                method: "".to_string(),
                url: url.to_string(),
                response,
            }),
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
        let authed_builder =
            self.builder_with_auth(unauthed_builder, &["ApiKeyAuth", "CookieAuth"]);
        let response = authed_builder.send().map_err(result::Error::Dispatch)?;
        let status_code = response.status().as_u16();
        match status_code {
            200 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Vec<ProjectMember>>(&response_text).unwrap();
                Ok(data)
            }
            400 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "GET".to_string(),
                    url: url.to_string(),
                    data: error_enums::ListApiProjectMembersErrors::Status400(data),
                })
            }
            401 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "GET".to_string(),
                    url: url.to_string(),
                    data: error_enums::ListApiProjectMembersErrors::Status401(data),
                })
            }
            404 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "GET".to_string(),
                    url: url.to_string(),
                    data: error_enums::ListApiProjectMembersErrors::Status404(data),
                })
            }
            500 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "GET".to_string(),
                    url: url.to_string(),
                    data: error_enums::ListApiProjectMembersErrors::Status500(data),
                })
            }
            _ => Err(result::Error::BlockingApiError {
                status_code,
                method: "".to_string(),
                url: url.to_string(),
                response,
            }),
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
        let authed_builder =
            self.builder_with_auth(unauthed_builder, &["ApiKeyAuth", "CookieAuth"]);
        let response = authed_builder.send().map_err(result::Error::Dispatch)?;
        let status_code = response.status().as_u16();
        match status_code {
            200 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Vec<ApiVersion>>(&response_text).unwrap();
                Ok(data)
            }
            400 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "GET".to_string(),
                    url: url.to_string(),
                    data: error_enums::ListApiVersionsErrors::Status400(data),
                })
            }
            401 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "GET".to_string(),
                    url: url.to_string(),
                    data: error_enums::ListApiVersionsErrors::Status401(data),
                })
            }
            404 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "GET".to_string(),
                    url: url.to_string(),
                    data: error_enums::ListApiVersionsErrors::Status404(data),
                })
            }
            500 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "GET".to_string(),
                    url: url.to_string(),
                    data: error_enums::ListApiVersionsErrors::Status500(data),
                })
            }
            _ => Err(result::Error::BlockingApiError {
                status_code,
                method: "".to_string(),
                url: url.to_string(),
                response,
            }),
        }
    }
    pub fn get_api_version(
        &self,
        request: GetApiVersionRequest,
    ) -> result::Result<ApiVersion, error_enums::GetApiVersionErrors> {
        let endpoint = format!(
            "/api_project/{}/version/{}",
            request.project_id_or_name, request.version_id_or_semver
        );
        let url = format!("{}{}", self.base_url, endpoint);
        let query_params: Vec<(&str, String)> = vec![];
        let unauthed_builder = ReqwestClient::default().get(&url).query(&query_params);
        let authed_builder =
            self.builder_with_auth(unauthed_builder, &["ApiKeyAuth", "CookieAuth"]);
        let response = authed_builder.send().map_err(result::Error::Dispatch)?;
        let status_code = response.status().as_u16();
        match status_code {
            200 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<ApiVersion>(&response_text).unwrap();
                Ok(data)
            }
            400 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "GET".to_string(),
                    url: url.to_string(),
                    data: error_enums::GetApiVersionErrors::Status400(data),
                })
            }
            401 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "GET".to_string(),
                    url: url.to_string(),
                    data: error_enums::GetApiVersionErrors::Status401(data),
                })
            }
            404 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "GET".to_string(),
                    url: url.to_string(),
                    data: error_enums::GetApiVersionErrors::Status404(data),
                })
            }
            500 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "GET".to_string(),
                    url: url.to_string(),
                    data: error_enums::GetApiVersionErrors::Status500(data),
                })
            }
            _ => Err(result::Error::BlockingApiError {
                status_code,
                method: "".to_string(),
                url: url.to_string(),
                response,
            }),
        }
    }
    pub fn get_api_version_openapi(
        &self,
        request: GetApiVersionOpenapiRequest,
    ) -> result::Result<OpenApi, error_enums::GetApiVersionOpenapiErrors> {
        let endpoint = format!(
            "/api_project/{}/version/{}/openapi",
            request.project_id_or_name, request.version_id_or_semver
        );
        let url = format!("{}{}", self.base_url, endpoint);
        let query_params: Vec<(&str, String)> = vec![];
        let unauthed_builder = ReqwestClient::default().get(&url).query(&query_params);
        let authed_builder =
            self.builder_with_auth(unauthed_builder, &["ApiKeyAuth", "CookieAuth"]);
        let response = authed_builder.send().map_err(result::Error::Dispatch)?;
        let status_code = response.status().as_u16();
        match status_code {
            200 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<OpenApi>(&response_text).unwrap();
                Ok(data)
            }
            400 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "GET".to_string(),
                    url: url.to_string(),
                    data: error_enums::GetApiVersionOpenapiErrors::Status400(data),
                })
            }
            401 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "GET".to_string(),
                    url: url.to_string(),
                    data: error_enums::GetApiVersionOpenapiErrors::Status401(data),
                })
            }
            404 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "GET".to_string(),
                    url: url.to_string(),
                    data: error_enums::GetApiVersionOpenapiErrors::Status404(data),
                })
            }
            500 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "GET".to_string(),
                    url: url.to_string(),
                    data: error_enums::GetApiVersionOpenapiErrors::Status500(data),
                })
            }
            _ => Err(result::Error::BlockingApiError {
                status_code,
                method: "".to_string(),
                url: url.to_string(),
                response,
            }),
        }
    }
    pub fn get_api_version_stats(
        &self,
        request: GetApiVersionStatsRequest,
    ) -> result::Result<Stats, error_enums::GetApiVersionStatsErrors> {
        let endpoint = format!(
            "/api_project/{}/version/{}/stats",
            request.project_id_or_name, request.version_id_or_semver
        );
        let url = format!("{}{}", self.base_url, endpoint);
        let query_params: Vec<(&str, String)> = vec![];
        let unauthed_builder = ReqwestClient::default().get(&url).query(&query_params);
        let authed_builder =
            self.builder_with_auth(unauthed_builder, &["ApiKeyAuth", "CookieAuth"]);
        let response = authed_builder.send().map_err(result::Error::Dispatch)?;
        let status_code = response.status().as_u16();
        match status_code {
            200 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Stats>(&response_text).unwrap();
                Ok(data)
            }
            400 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "GET".to_string(),
                    url: url.to_string(),
                    data: error_enums::GetApiVersionStatsErrors::Status400(data),
                })
            }
            401 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "GET".to_string(),
                    url: url.to_string(),
                    data: error_enums::GetApiVersionStatsErrors::Status401(data),
                })
            }
            404 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "GET".to_string(),
                    url: url.to_string(),
                    data: error_enums::GetApiVersionStatsErrors::Status404(data),
                })
            }
            500 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "GET".to_string(),
                    url: url.to_string(),
                    data: error_enums::GetApiVersionStatsErrors::Status500(data),
                })
            }
            _ => Err(result::Error::BlockingApiError {
                status_code,
                method: "".to_string(),
                url: url.to_string(),
                response,
            }),
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
            400 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "GET".to_string(),
                    url: url.to_string(),
                    data: error_enums::ExchangeCodeForKeyErrors::Status400(data),
                })
            }
            401 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "GET".to_string(),
                    url: url.to_string(),
                    data: error_enums::ExchangeCodeForKeyErrors::Status401(data),
                })
            }
            404 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "GET".to_string(),
                    url: url.to_string(),
                    data: error_enums::ExchangeCodeForKeyErrors::Status404(data),
                })
            }
            500 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "GET".to_string(),
                    url: url.to_string(),
                    data: error_enums::ExchangeCodeForKeyErrors::Status500(data),
                })
            }
            _ => Err(result::Error::BlockingApiError {
                status_code,
                method: "".to_string(),
                url: url.to_string(),
                response,
            }),
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
                let data = serde_json::from_str::<serde_json::Value>(&response_text).unwrap();
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
            _ => Err(result::Error::BlockingApiError {
                status_code,
                method: "".to_string(),
                url: url.to_string(),
                response,
            }),
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
                let data = serde_json::from_str::<serde_json::Value>(&response_text).unwrap();
                Ok(data)
            }
            _ => Err(result::Error::BlockingApiError {
                status_code,
                method: "".to_string(),
                url: url.to_string(),
                response,
            }),
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
                let data = serde_json::from_str::<Vec<CliUpdate>>(&response_text).unwrap();
                Ok(data)
            }
            400 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "GET".to_string(),
                    url: url.to_string(),
                    data: error_enums::CliCheckUpdatesErrors::Status400(data),
                })
            }
            401 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "GET".to_string(),
                    url: url.to_string(),
                    data: error_enums::CliCheckUpdatesErrors::Status401(data),
                })
            }
            404 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "GET".to_string(),
                    url: url.to_string(),
                    data: error_enums::CliCheckUpdatesErrors::Status404(data),
                })
            }
            500 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "GET".to_string(),
                    url: url.to_string(),
                    data: error_enums::CliCheckUpdatesErrors::Status500(data),
                })
            }
            _ => Err(result::Error::BlockingApiError {
                status_code,
                method: "".to_string(),
                url: url.to_string(),
                response,
            }),
        }
    }
    pub fn list_doc_projects(
        &self,
    ) -> result::Result<Vec<DocProject>, error_enums::ListDocProjectsErrors> {
        let endpoint = "/doc_project";
        let url = format!("{}{}", self.base_url, endpoint);
        let query_params: Vec<(&str, String)> = vec![];
        let unauthed_builder = ReqwestClient::default().get(&url).query(&query_params);
        let authed_builder =
            self.builder_with_auth(unauthed_builder, &["ApiKeyAuth", "CookieAuth"]);
        let response = authed_builder.send().map_err(result::Error::Dispatch)?;
        let status_code = response.status().as_u16();
        match status_code {
            200 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Vec<DocProject>>(&response_text).unwrap();
                Ok(data)
            }
            400 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "GET".to_string(),
                    url: url.to_string(),
                    data: error_enums::ListDocProjectsErrors::Status400(data),
                })
            }
            401 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "GET".to_string(),
                    url: url.to_string(),
                    data: error_enums::ListDocProjectsErrors::Status401(data),
                })
            }
            404 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "GET".to_string(),
                    url: url.to_string(),
                    data: error_enums::ListDocProjectsErrors::Status404(data),
                })
            }
            500 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "GET".to_string(),
                    url: url.to_string(),
                    data: error_enums::ListDocProjectsErrors::Status500(data),
                })
            }
            _ => Err(result::Error::BlockingApiError {
                status_code,
                method: "".to_string(),
                url: url.to_string(),
                response,
            }),
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
        let authed_builder =
            self.builder_with_auth(unauthed_builder, &["ApiKeyAuth", "CookieAuth"]);
        let response = authed_builder.send().map_err(result::Error::Dispatch)?;
        let status_code = response.status().as_u16();
        match status_code {
            200 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<DocProject>(&response_text).unwrap();
                Ok(data)
            }
            400 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "GET".to_string(),
                    url: url.to_string(),
                    data: error_enums::GetDocProjectErrors::Status400(data),
                })
            }
            401 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "GET".to_string(),
                    url: url.to_string(),
                    data: error_enums::GetDocProjectErrors::Status401(data),
                })
            }
            404 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "GET".to_string(),
                    url: url.to_string(),
                    data: error_enums::GetDocProjectErrors::Status404(data),
                })
            }
            500 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "GET".to_string(),
                    url: url.to_string(),
                    data: error_enums::GetDocProjectErrors::Status500(data),
                })
            }
            _ => Err(result::Error::BlockingApiError {
                status_code,
                method: "".to_string(),
                url: url.to_string(),
                response,
            }),
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
        let authed_builder =
            self.builder_with_auth(unauthed_builder, &["ApiKeyAuth", "CookieAuth"]);
        let response = authed_builder.send().map_err(result::Error::Dispatch)?;
        let status_code = response.status().as_u16();
        match status_code {
            200 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Vec<Deployment>>(&response_text).unwrap();
                Ok(data)
            }
            400 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "GET".to_string(),
                    url: url.to_string(),
                    data: error_enums::ListDeploymentsErrors::Status400(data),
                })
            }
            401 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "GET".to_string(),
                    url: url.to_string(),
                    data: error_enums::ListDeploymentsErrors::Status401(data),
                })
            }
            404 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "GET".to_string(),
                    url: url.to_string(),
                    data: error_enums::ListDeploymentsErrors::Status404(data),
                })
            }
            500 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "GET".to_string(),
                    url: url.to_string(),
                    data: error_enums::ListDeploymentsErrors::Status500(data),
                })
            }
            _ => Err(result::Error::BlockingApiError {
                status_code,
                method: "".to_string(),
                url: url.to_string(),
                response,
            }),
        }
    }
    /// Retrieves single deployment
    pub fn get_deployment(
        &self,
        request: GetDeploymentRequest,
    ) -> result::Result<Deployment, error_enums::GetDeploymentErrors> {
        let endpoint = format!(
            "/doc_project/{}/deployment/{}",
            request.project_id_or_name, request.deployment_id
        );
        let url = format!("{}{}", self.base_url, endpoint);
        let query_params: Vec<(&str, String)> = vec![];
        let unauthed_builder = ReqwestClient::default().get(&url).query(&query_params);
        let authed_builder =
            self.builder_with_auth(unauthed_builder, &["ApiKeyAuth", "CookieAuth"]);
        let response = authed_builder.send().map_err(result::Error::Dispatch)?;
        let status_code = response.status().as_u16();
        match status_code {
            200 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Deployment>(&response_text).unwrap();
                Ok(data)
            }
            400 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "GET".to_string(),
                    url: url.to_string(),
                    data: error_enums::GetDeploymentErrors::Status400(data),
                })
            }
            401 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "GET".to_string(),
                    url: url.to_string(),
                    data: error_enums::GetDeploymentErrors::Status401(data),
                })
            }
            404 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "GET".to_string(),
                    url: url.to_string(),
                    data: error_enums::GetDeploymentErrors::Status404(data),
                })
            }
            500 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "GET".to_string(),
                    url: url.to_string(),
                    data: error_enums::GetDeploymentErrors::Status500(data),
                })
            }
            _ => Err(result::Error::BlockingApiError {
                status_code,
                method: "".to_string(),
                url: url.to_string(),
                response,
            }),
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
        let authed_builder =
            self.builder_with_auth(unauthed_builder, &["ApiKeyAuth", "CookieAuth"]);
        let response = authed_builder.send().map_err(result::Error::Dispatch)?;
        let status_code = response.status().as_u16();
        match status_code {
            200 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Vec<ProjectMember>>(&response_text).unwrap();
                Ok(data)
            }
            400 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "GET".to_string(),
                    url: url.to_string(),
                    data: error_enums::ListDocProjectMembersErrors::Status400(data),
                })
            }
            401 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "GET".to_string(),
                    url: url.to_string(),
                    data: error_enums::ListDocProjectMembersErrors::Status401(data),
                })
            }
            404 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "GET".to_string(),
                    url: url.to_string(),
                    data: error_enums::ListDocProjectMembersErrors::Status404(data),
                })
            }
            500 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "GET".to_string(),
                    url: url.to_string(),
                    data: error_enums::ListDocProjectMembersErrors::Status500(data),
                })
            }
            _ => Err(result::Error::BlockingApiError {
                status_code,
                method: "".to_string(),
                url: url.to_string(),
                response,
            }),
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
        let authed_builder =
            self.builder_with_auth(unauthed_builder, &["ApiKeyAuth", "CookieAuth"]);
        let response = authed_builder.send().map_err(result::Error::Dispatch)?;
        let status_code = response.status().as_u16();
        match status_code {
            200 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<bool>(&response_text).unwrap();
                Ok(data)
            }
            400 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "GET".to_string(),
                    url: url.to_string(),
                    data: error_enums::CheckPreviewErrors::Status400(data),
                })
            }
            401 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "GET".to_string(),
                    url: url.to_string(),
                    data: error_enums::CheckPreviewErrors::Status401(data),
                })
            }
            404 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "GET".to_string(),
                    url: url.to_string(),
                    data: error_enums::CheckPreviewErrors::Status404(data),
                })
            }
            500 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "GET".to_string(),
                    url: url.to_string(),
                    data: error_enums::CheckPreviewErrors::Status500(data),
                })
            }
            _ => Err(result::Error::BlockingApiError {
                status_code,
                method: "".to_string(),
                url: url.to_string(),
                response,
            }),
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
        let authed_builder =
            self.builder_with_auth(unauthed_builder, &["ApiKeyAuth", "CookieAuth"]);
        let response = authed_builder.send().map_err(result::Error::Dispatch)?;
        let status_code = response.status().as_u16();
        match status_code {
            200 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Theme>(&response_text).unwrap();
                Ok(data)
            }
            400 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "GET".to_string(),
                    url: url.to_string(),
                    data: error_enums::GetDocProjectThemeErrors::Status400(data),
                })
            }
            401 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "GET".to_string(),
                    url: url.to_string(),
                    data: error_enums::GetDocProjectThemeErrors::Status401(data),
                })
            }
            404 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "GET".to_string(),
                    url: url.to_string(),
                    data: error_enums::GetDocProjectThemeErrors::Status404(data),
                })
            }
            500 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "GET".to_string(),
                    url: url.to_string(),
                    data: error_enums::GetDocProjectThemeErrors::Status500(data),
                })
            }
            _ => Err(result::Error::BlockingApiError {
                status_code,
                method: "".to_string(),
                url: url.to_string(),
                response,
            }),
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
        let authed_builder =
            self.builder_with_auth(unauthed_builder, &["ApiKeyAuth", "CookieAuth"]);
        let response = authed_builder.send().map_err(result::Error::Dispatch)?;
        let status_code = response.status().as_u16();
        match status_code {
            200 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Vec<DocVersion>>(&response_text).unwrap();
                Ok(data)
            }
            400 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "GET".to_string(),
                    url: url.to_string(),
                    data: error_enums::ListDocVersionsErrors::Status400(data),
                })
            }
            401 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "GET".to_string(),
                    url: url.to_string(),
                    data: error_enums::ListDocVersionsErrors::Status401(data),
                })
            }
            404 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "GET".to_string(),
                    url: url.to_string(),
                    data: error_enums::ListDocVersionsErrors::Status404(data),
                })
            }
            500 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "GET".to_string(),
                    url: url.to_string(),
                    data: error_enums::ListDocVersionsErrors::Status500(data),
                })
            }
            _ => Err(result::Error::BlockingApiError {
                status_code,
                method: "".to_string(),
                url: url.to_string(),
                response,
            }),
        }
    }
    pub fn get_doc_version(
        &self,
        request: GetDocVersionRequest,
    ) -> result::Result<DocVersion, error_enums::GetDocVersionErrors> {
        let endpoint = format!(
            "/doc_project/{}/version/{}",
            request.project_id_or_name, request.version_id
        );
        let url = format!("{}{}", self.base_url, endpoint);
        let query_params: Vec<(&str, String)> = vec![];
        let unauthed_builder = ReqwestClient::default().get(&url).query(&query_params);
        let authed_builder =
            self.builder_with_auth(unauthed_builder, &["ApiKeyAuth", "CookieAuth"]);
        let response = authed_builder.send().map_err(result::Error::Dispatch)?;
        let status_code = response.status().as_u16();
        match status_code {
            200 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<DocVersion>(&response_text).unwrap();
                Ok(data)
            }
            400 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "GET".to_string(),
                    url: url.to_string(),
                    data: error_enums::GetDocVersionErrors::Status400(data),
                })
            }
            401 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "GET".to_string(),
                    url: url.to_string(),
                    data: error_enums::GetDocVersionErrors::Status401(data),
                })
            }
            404 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "GET".to_string(),
                    url: url.to_string(),
                    data: error_enums::GetDocVersionErrors::Status404(data),
                })
            }
            500 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "GET".to_string(),
                    url: url.to_string(),
                    data: error_enums::GetDocVersionErrors::Status500(data),
                })
            }
            _ => Err(result::Error::BlockingApiError {
                status_code,
                method: "".to_string(),
                url: url.to_string(),
                response,
            }),
        }
    }
    pub fn list_guides(
        &self,
        request: ListGuidesRequest,
    ) -> result::Result<Vec<GuideWithChildren>, error_enums::ListGuidesErrors> {
        let endpoint = format!(
            "/doc_project/{}/version/{}/guide",
            request.project_id_or_name, request.version_id
        );
        let url = format!("{}{}", self.base_url, endpoint);
        let query_params: Vec<(&str, String)> = vec![];
        let unauthed_builder = ReqwestClient::default().get(&url).query(&query_params);
        let authed_builder =
            self.builder_with_auth(unauthed_builder, &["ApiKeyAuth", "CookieAuth"]);
        let response = authed_builder.send().map_err(result::Error::Dispatch)?;
        let status_code = response.status().as_u16();
        match status_code {
            200 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Vec<GuideWithChildren>>(&response_text).unwrap();
                Ok(data)
            }
            400 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "GET".to_string(),
                    url: url.to_string(),
                    data: error_enums::ListGuidesErrors::Status400(data),
                })
            }
            401 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "GET".to_string(),
                    url: url.to_string(),
                    data: error_enums::ListGuidesErrors::Status401(data),
                })
            }
            404 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "GET".to_string(),
                    url: url.to_string(),
                    data: error_enums::ListGuidesErrors::Status404(data),
                })
            }
            500 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "GET".to_string(),
                    url: url.to_string(),
                    data: error_enums::ListGuidesErrors::Status500(data),
                })
            }
            _ => Err(result::Error::BlockingApiError {
                status_code,
                method: "".to_string(),
                url: url.to_string(),
                response,
            }),
        }
    }
    pub fn get_guide(
        &self,
        request: GetGuideRequest,
    ) -> result::Result<Guide, error_enums::GetGuideErrors> {
        let endpoint = format!(
            "/doc_project/{}/version/{}/guide/{}",
            request.project_id_or_name, request.version_id, request.guide_id
        );
        let url = format!("{}{}", self.base_url, endpoint);
        let query_params: Vec<(&str, String)> = vec![];
        let unauthed_builder = ReqwestClient::default().get(&url).query(&query_params);
        let authed_builder =
            self.builder_with_auth(unauthed_builder, &["ApiKeyAuth", "CookieAuth"]);
        let response = authed_builder.send().map_err(result::Error::Dispatch)?;
        let status_code = response.status().as_u16();
        match status_code {
            200 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Guide>(&response_text).unwrap();
                Ok(data)
            }
            400 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "GET".to_string(),
                    url: url.to_string(),
                    data: error_enums::GetGuideErrors::Status400(data),
                })
            }
            401 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "GET".to_string(),
                    url: url.to_string(),
                    data: error_enums::GetGuideErrors::Status401(data),
                })
            }
            404 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "GET".to_string(),
                    url: url.to_string(),
                    data: error_enums::GetGuideErrors::Status404(data),
                })
            }
            500 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "GET".to_string(),
                    url: url.to_string(),
                    data: error_enums::GetGuideErrors::Status500(data),
                })
            }
            _ => Err(result::Error::BlockingApiError {
                status_code,
                method: "".to_string(),
                url: url.to_string(),
                response,
            }),
        }
    }
    pub fn get_guide_content(
        &self,
        request: GetGuideContentRequest,
    ) -> result::Result<GuideContent, error_enums::GetGuideContentErrors> {
        let endpoint = format!(
            "/doc_project/{}/version/{}/guide/{}/content",
            request.project_id_or_name, request.version_id, request.guide_id
        );
        let url = format!("{}{}", self.base_url, endpoint);
        let query_params: Vec<(&str, String)> = vec![];
        let unauthed_builder = ReqwestClient::default().get(&url).query(&query_params);
        let authed_builder =
            self.builder_with_auth(unauthed_builder, &["ApiKeyAuth", "CookieAuth"]);
        let response = authed_builder.send().map_err(result::Error::Dispatch)?;
        let status_code = response.status().as_u16();
        match status_code {
            200 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<GuideContent>(&response_text).unwrap();
                Ok(data)
            }
            400 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "GET".to_string(),
                    url: url.to_string(),
                    data: error_enums::GetGuideContentErrors::Status400(data),
                })
            }
            401 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "GET".to_string(),
                    url: url.to_string(),
                    data: error_enums::GetGuideContentErrors::Status401(data),
                })
            }
            404 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "GET".to_string(),
                    url: url.to_string(),
                    data: error_enums::GetGuideContentErrors::Status404(data),
                })
            }
            500 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "GET".to_string(),
                    url: url.to_string(),
                    data: error_enums::GetGuideContentErrors::Status500(data),
                })
            }
            _ => Err(result::Error::BlockingApiError {
                status_code,
                method: "".to_string(),
                url: url.to_string(),
                response,
            }),
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
        let authed_builder =
            self.builder_with_auth(unauthed_builder, &["ApiKeyAuth", "CookieAuth"]);
        let response = authed_builder.send().map_err(result::Error::Dispatch)?;
        let status_code = response.status().as_u16();
        match status_code {
            200 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Organization>(&response_text).unwrap();
                Ok(data)
            }
            _ => Err(result::Error::BlockingApiError {
                status_code,
                method: "".to_string(),
                url: url.to_string(),
                response,
            }),
        }
    }
    /// Get all assets for an organization
    pub fn list_assets(
        &self,
        request: ListAssetsRequest,
    ) -> result::Result<Vec<ListAssetsPage>, error_enums::ListAssetsErrors> {
        let endpoint = "/organization/asset";
        let url = format!("{}{}", self.base_url, endpoint);
        let mut query_params: Vec<(&str, String)> = vec![];
        if let Some(name) = request.name {
            query_params.push(("name", name.to_string()));
        }
        if let Some(page) = request.page {
            query_params.push(("page", page.to_string()));
        }
        let unauthed_builder = ReqwestClient::default().get(&url).query(&query_params);
        let authed_builder =
            self.builder_with_auth(unauthed_builder, &["ApiKeyAuth", "CookieAuth"]);
        let response = authed_builder.send().map_err(result::Error::Dispatch)?;
        let status_code = response.status().as_u16();
        match status_code {
            200 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Vec<ListAssetsPage>>(&response_text).unwrap();
                Ok(data)
            }
            400 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "GET".to_string(),
                    url: url.to_string(),
                    data: error_enums::ListAssetsErrors::Status400(data),
                })
            }
            401 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "GET".to_string(),
                    url: url.to_string(),
                    data: error_enums::ListAssetsErrors::Status401(data),
                })
            }
            404 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "GET".to_string(),
                    url: url.to_string(),
                    data: error_enums::ListAssetsErrors::Status404(data),
                })
            }
            500 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "GET".to_string(),
                    url: url.to_string(),
                    data: error_enums::ListAssetsErrors::Status500(data),
                })
            }
            _ => Err(result::Error::BlockingApiError {
                status_code,
                method: "".to_string(),
                url: url.to_string(),
                response,
            }),
        }
    }
    /// Get users in the organization
    pub fn list_organization_members(
        &self,
    ) -> result::Result<Vec<OrganizationMember>, error_enums::ListOrganizationMembersErrors> {
        let endpoint = "/organization/members";
        let url = format!("{}{}", self.base_url, endpoint);
        let query_params: Vec<(&str, String)> = vec![];
        let unauthed_builder = ReqwestClient::default().get(&url).query(&query_params);
        let authed_builder =
            self.builder_with_auth(unauthed_builder, &["ApiKeyAuth", "CookieAuth"]);
        let response = authed_builder.send().map_err(result::Error::Dispatch)?;
        let status_code = response.status().as_u16();
        match status_code {
            200 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Vec<OrganizationMember>>(&response_text).unwrap();
                Ok(data)
            }
            400 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "GET".to_string(),
                    url: url.to_string(),
                    data: error_enums::ListOrganizationMembersErrors::Status400(data),
                })
            }
            401 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "GET".to_string(),
                    url: url.to_string(),
                    data: error_enums::ListOrganizationMembersErrors::Status401(data),
                })
            }
            404 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "GET".to_string(),
                    url: url.to_string(),
                    data: error_enums::ListOrganizationMembersErrors::Status404(data),
                })
            }
            500 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "GET".to_string(),
                    url: url.to_string(),
                    data: error_enums::ListOrganizationMembersErrors::Status500(data),
                })
            }
            _ => Err(result::Error::BlockingApiError {
                status_code,
                method: "".to_string(),
                url: url.to_string(),
                response,
            }),
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
        let authed_builder =
            self.builder_with_auth(unauthed_builder, &["ApiKeyAuth", "CookieAuth"]);
        let response = authed_builder.send().map_err(result::Error::Dispatch)?;
        let status_code = response.status().as_u16();
        match status_code {
            200 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Theme>(&response_text).unwrap();
                Ok(data)
            }
            400 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "GET".to_string(),
                    url: url.to_string(),
                    data: error_enums::GetOrganizationThemeErrors::Status400(data),
                })
            }
            401 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "GET".to_string(),
                    url: url.to_string(),
                    data: error_enums::GetOrganizationThemeErrors::Status401(data),
                })
            }
            404 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "GET".to_string(),
                    url: url.to_string(),
                    data: error_enums::GetOrganizationThemeErrors::Status404(data),
                })
            }
            500 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "GET".to_string(),
                    url: url.to_string(),
                    data: error_enums::GetOrganizationThemeErrors::Status500(data),
                })
            }
            _ => Err(result::Error::BlockingApiError {
                status_code,
                method: "".to_string(),
                url: url.to_string(),
                response,
            }),
        }
    }
    pub fn list_sdks(
        &self,
        request: ListSdksRequest,
    ) -> result::Result<Vec<SdkProject>, error_enums::ListSdksErrors> {
        let endpoint = format!("/sdk/{}", request.api_id_or_name);
        let url = format!("{}{}", self.base_url, endpoint);
        let query_params: Vec<(&str, String)> = vec![];
        let unauthed_builder = ReqwestClient::default().get(&url).query(&query_params);
        let authed_builder =
            self.builder_with_auth(unauthed_builder, &["ApiKeyAuth", "CookieAuth"]);
        let response = authed_builder.send().map_err(result::Error::Dispatch)?;
        let status_code = response.status().as_u16();
        match status_code {
            200 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Vec<SdkProject>>(&response_text).unwrap();
                Ok(data)
            }
            _ => Err(result::Error::BlockingApiError {
                status_code,
                method: "".to_string(),
                url: url.to_string(),
                response,
            }),
        }
    }
    pub fn get_current_user(&self) -> result::Result<User, error_enums::GetCurrentUserErrors> {
        let endpoint = "/user/me";
        let url = format!("{}{}", self.base_url, endpoint);
        let query_params: Vec<(&str, String)> = vec![];
        let unauthed_builder = ReqwestClient::default().get(&url).query(&query_params);
        let authed_builder =
            self.builder_with_auth(unauthed_builder, &["ApiKeyAuth", "CookieAuth"]);
        let response = authed_builder.send().map_err(result::Error::Dispatch)?;
        let status_code = response.status().as_u16();
        match status_code {
            200 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<User>(&response_text).unwrap();
                Ok(data)
            }
            400 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "GET".to_string(),
                    url: url.to_string(),
                    data: error_enums::GetCurrentUserErrors::Status400(data),
                })
            }
            401 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "GET".to_string(),
                    url: url.to_string(),
                    data: error_enums::GetCurrentUserErrors::Status401(data),
                })
            }
            404 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "GET".to_string(),
                    url: url.to_string(),
                    data: error_enums::GetCurrentUserErrors::Status404(data),
                })
            }
            500 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "GET".to_string(),
                    url: url.to_string(),
                    data: error_enums::GetCurrentUserErrors::Status500(data),
                })
            }
            _ => Err(result::Error::BlockingApiError {
                status_code,
                method: "".to_string(),
                url: url.to_string(),
                response,
            }),
        }
    }
    pub fn get_api_key(&self) -> result::Result<UserApiKey, error_enums::GetApiKeyErrors> {
        let endpoint = "/user/me/api_key";
        let url = format!("{}{}", self.base_url, endpoint);
        let query_params: Vec<(&str, String)> = vec![];
        let unauthed_builder = ReqwestClient::default().get(&url).query(&query_params);
        let authed_builder =
            self.builder_with_auth(unauthed_builder, &["ApiKeyAuth", "CookieAuth"]);
        let response = authed_builder.send().map_err(result::Error::Dispatch)?;
        let status_code = response.status().as_u16();
        match status_code {
            200 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<UserApiKey>(&response_text).unwrap();
                Ok(data)
            }
            400 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "GET".to_string(),
                    url: url.to_string(),
                    data: error_enums::GetApiKeyErrors::Status400(data),
                })
            }
            401 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "GET".to_string(),
                    url: url.to_string(),
                    data: error_enums::GetApiKeyErrors::Status401(data),
                })
            }
            404 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "GET".to_string(),
                    url: url.to_string(),
                    data: error_enums::GetApiKeyErrors::Status404(data),
                })
            }
            500 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "GET".to_string(),
                    url: url.to_string(),
                    data: error_enums::GetApiKeyErrors::Status500(data),
                })
            }
            _ => Err(result::Error::BlockingApiError {
                status_code,
                method: "".to_string(),
                url: url.to_string(),
                response,
            }),
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
        if let Some(project_id) = request.project_id {
            query_params.push(("project_id", project_id.to_string()));
        }
        let unauthed_builder = ReqwestClient::default().get(&url).query(&query_params);
        let authed_builder =
            self.builder_with_auth(unauthed_builder, &["ApiKeyAuth", "CookieAuth"]);
        let response = authed_builder.send().map_err(result::Error::Dispatch)?;
        let status_code = response.status().as_u16();
        match status_code {
            200 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<UserProjectRole>(&response_text).unwrap();
                Ok(data)
            }
            400 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "GET".to_string(),
                    url: url.to_string(),
                    data: error_enums::GetUserProjectRoleErrors::Status400(data),
                })
            }
            401 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "GET".to_string(),
                    url: url.to_string(),
                    data: error_enums::GetUserProjectRoleErrors::Status401(data),
                })
            }
            404 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "GET".to_string(),
                    url: url.to_string(),
                    data: error_enums::GetUserProjectRoleErrors::Status404(data),
                })
            }
            500 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "GET".to_string(),
                    url: url.to_string(),
                    data: error_enums::GetUserProjectRoleErrors::Status500(data),
                })
            }
            _ => Err(result::Error::BlockingApiError {
                status_code,
                method: "".to_string(),
                url: url.to_string(),
                response,
            }),
        }
    }
    pub fn get_service_accounts(
        &self,
    ) -> result::Result<Vec<ServiceAccount>, error_enums::GetServiceAccountsErrors> {
        let endpoint = "/user/service_account";
        let url = format!("{}{}", self.base_url, endpoint);
        let query_params: Vec<(&str, String)> = vec![];
        let unauthed_builder = ReqwestClient::default().get(&url).query(&query_params);
        let authed_builder =
            self.builder_with_auth(unauthed_builder, &["ApiKeyAuth", "CookieAuth"]);
        let response = authed_builder.send().map_err(result::Error::Dispatch)?;
        let status_code = response.status().as_u16();
        match status_code {
            200 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Vec<ServiceAccount>>(&response_text).unwrap();
                Ok(data)
            }
            400 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "GET".to_string(),
                    url: url.to_string(),
                    data: error_enums::GetServiceAccountsErrors::Status400(data),
                })
            }
            401 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "GET".to_string(),
                    url: url.to_string(),
                    data: error_enums::GetServiceAccountsErrors::Status401(data),
                })
            }
            404 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "GET".to_string(),
                    url: url.to_string(),
                    data: error_enums::GetServiceAccountsErrors::Status404(data),
                })
            }
            500 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "GET".to_string(),
                    url: url.to_string(),
                    data: error_enums::GetServiceAccountsErrors::Status500(data),
                })
            }
            _ => Err(result::Error::BlockingApiError {
                status_code,
                method: "".to_string(),
                url: url.to_string(),
                response,
            }),
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
        let authed_builder =
            self.builder_with_auth(unauthed_builder, &["ApiKeyAuth", "CookieAuth"]);
        let request_body: serde_json::Value =
            serde_json::to_value(request.data).map_err(result::Error::Serialize)?;
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
            400 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "PATCH".to_string(),
                    url: url.to_string(),
                    data: error_enums::UpdateApiLinkErrors::Status400(data),
                })
            }
            401 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "PATCH".to_string(),
                    url: url.to_string(),
                    data: error_enums::UpdateApiLinkErrors::Status401(data),
                })
            }
            404 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "PATCH".to_string(),
                    url: url.to_string(),
                    data: error_enums::UpdateApiLinkErrors::Status404(data),
                })
            }
            500 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "PATCH".to_string(),
                    url: url.to_string(),
                    data: error_enums::UpdateApiLinkErrors::Status500(data),
                })
            }
            _ => Err(result::Error::BlockingApiError {
                status_code,
                method: "".to_string(),
                url: url.to_string(),
                response,
            }),
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
        let authed_builder =
            self.builder_with_auth(unauthed_builder, &["ApiKeyAuth", "CookieAuth"]);
        let request_body: serde_json::Value =
            serde_json::to_value(request.data).map_err(result::Error::Serialize)?;
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
            400 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "PATCH".to_string(),
                    url: url.to_string(),
                    data: error_enums::UpdateApiLinkGroupErrors::Status400(data),
                })
            }
            401 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "PATCH".to_string(),
                    url: url.to_string(),
                    data: error_enums::UpdateApiLinkGroupErrors::Status401(data),
                })
            }
            404 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "PATCH".to_string(),
                    url: url.to_string(),
                    data: error_enums::UpdateApiLinkGroupErrors::Status404(data),
                })
            }
            500 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "PATCH".to_string(),
                    url: url.to_string(),
                    data: error_enums::UpdateApiLinkGroupErrors::Status500(data),
                })
            }
            _ => Err(result::Error::BlockingApiError {
                status_code,
                method: "".to_string(),
                url: url.to_string(),
                response,
            }),
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
        let authed_builder =
            self.builder_with_auth(unauthed_builder, &["ApiKeyAuth", "CookieAuth"]);
        let request_body: serde_json::Value =
            serde_json::to_value(request.data).map_err(result::Error::Serialize)?;
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
            400 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "PATCH".to_string(),
                    url: url.to_string(),
                    data: error_enums::UpdateApiProjectErrors::Status400(data),
                })
            }
            401 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "PATCH".to_string(),
                    url: url.to_string(),
                    data: error_enums::UpdateApiProjectErrors::Status401(data),
                })
            }
            404 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "PATCH".to_string(),
                    url: url.to_string(),
                    data: error_enums::UpdateApiProjectErrors::Status404(data),
                })
            }
            500 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "PATCH".to_string(),
                    url: url.to_string(),
                    data: error_enums::UpdateApiProjectErrors::Status500(data),
                })
            }
            _ => Err(result::Error::BlockingApiError {
                status_code,
                method: "".to_string(),
                url: url.to_string(),
                response,
            }),
        }
    }
    pub fn update_api_version(
        &self,
        request: UpdateApiVersionRequest,
    ) -> result::Result<ApiVersion, error_enums::UpdateApiVersionErrors> {
        let endpoint = format!(
            "/api_project/{}/version/{}",
            request.project_id_or_name, request.version_id_or_semver
        );
        let url = format!("{}{}", self.base_url, endpoint);
        let query_params: Vec<(&str, String)> = vec![];
        let unauthed_builder = ReqwestClient::default().patch(&url).query(&query_params);
        let authed_builder =
            self.builder_with_auth(unauthed_builder, &["ApiKeyAuth", "CookieAuth"]);
        let request_body: serde_json::Value =
            serde_json::to_value(request.data).map_err(result::Error::Serialize)?;
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
            400 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "PATCH".to_string(),
                    url: url.to_string(),
                    data: error_enums::UpdateApiVersionErrors::Status400(data),
                })
            }
            401 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "PATCH".to_string(),
                    url: url.to_string(),
                    data: error_enums::UpdateApiVersionErrors::Status401(data),
                })
            }
            404 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "PATCH".to_string(),
                    url: url.to_string(),
                    data: error_enums::UpdateApiVersionErrors::Status404(data),
                })
            }
            500 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "PATCH".to_string(),
                    url: url.to_string(),
                    data: error_enums::UpdateApiVersionErrors::Status500(data),
                })
            }
            _ => Err(result::Error::BlockingApiError {
                status_code,
                method: "".to_string(),
                url: url.to_string(),
                response,
            }),
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
        let authed_builder =
            self.builder_with_auth(unauthed_builder, &["ApiKeyAuth", "CookieAuth"]);
        let request_body: serde_json::Value =
            serde_json::to_value(request.data).map_err(result::Error::Serialize)?;
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
            400 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "PATCH".to_string(),
                    url: url.to_string(),
                    data: error_enums::UpdateDocProjectErrors::Status400(data),
                })
            }
            401 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "PATCH".to_string(),
                    url: url.to_string(),
                    data: error_enums::UpdateDocProjectErrors::Status401(data),
                })
            }
            404 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "PATCH".to_string(),
                    url: url.to_string(),
                    data: error_enums::UpdateDocProjectErrors::Status404(data),
                })
            }
            500 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "PATCH".to_string(),
                    url: url.to_string(),
                    data: error_enums::UpdateDocProjectErrors::Status500(data),
                })
            }
            _ => Err(result::Error::BlockingApiError {
                status_code,
                method: "".to_string(),
                url: url.to_string(),
                response,
            }),
        }
    }
    pub fn update_guide(
        &self,
        request: UpdateGuideRequest,
    ) -> result::Result<Guide, error_enums::UpdateGuideErrors> {
        let endpoint = format!(
            "/doc_project/{}/version/{}/guide/{}",
            request.project_id_or_name, request.version_id, request.guide_id
        );
        let url = format!("{}{}", self.base_url, endpoint);
        let query_params: Vec<(&str, String)> = vec![];
        let unauthed_builder = ReqwestClient::default().patch(&url).query(&query_params);
        let authed_builder =
            self.builder_with_auth(unauthed_builder, &["ApiKeyAuth", "CookieAuth"]);
        let request_body: serde_json::Value =
            serde_json::to_value(request.data).map_err(result::Error::Serialize)?;
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
            400 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "PATCH".to_string(),
                    url: url.to_string(),
                    data: error_enums::UpdateGuideErrors::Status400(data),
                })
            }
            401 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "PATCH".to_string(),
                    url: url.to_string(),
                    data: error_enums::UpdateGuideErrors::Status401(data),
                })
            }
            404 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "PATCH".to_string(),
                    url: url.to_string(),
                    data: error_enums::UpdateGuideErrors::Status404(data),
                })
            }
            500 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "PATCH".to_string(),
                    url: url.to_string(),
                    data: error_enums::UpdateGuideErrors::Status500(data),
                })
            }
            _ => Err(result::Error::BlockingApiError {
                status_code,
                method: "".to_string(),
                url: url.to_string(),
                response,
            }),
        }
    }
    /// Update an asset in your organization
    pub fn update_asset(
        &self,
        request: UpdateAssetRequest,
    ) -> result::Result<Asset, error_enums::UpdateAssetErrors> {
        let endpoint = format!("/organization/asset/{}", request.asset_id);
        let url = format!("{}{}", self.base_url, endpoint);
        let query_params: Vec<(&str, String)> = vec![];
        let unauthed_builder = ReqwestClient::default().patch(&url).query(&query_params);
        let authed_builder =
            self.builder_with_auth(unauthed_builder, &["ApiKeyAuth", "CookieAuth"]);
        let request_body: serde_json::Value =
            serde_json::to_value(request.data).map_err(result::Error::Serialize)?;
        let response = authed_builder
            .json(&request_body)
            .send()
            .map_err(result::Error::Dispatch)?;
        let status_code = response.status().as_u16();
        match status_code {
            200 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Asset>(&response_text).unwrap();
                Ok(data)
            }
            400 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "PATCH".to_string(),
                    url: url.to_string(),
                    data: error_enums::UpdateAssetErrors::Status400(data),
                })
            }
            401 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "PATCH".to_string(),
                    url: url.to_string(),
                    data: error_enums::UpdateAssetErrors::Status401(data),
                })
            }
            404 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "PATCH".to_string(),
                    url: url.to_string(),
                    data: error_enums::UpdateAssetErrors::Status404(data),
                })
            }
            500 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "PATCH".to_string(),
                    url: url.to_string(),
                    data: error_enums::UpdateAssetErrors::Status500(data),
                })
            }
            _ => Err(result::Error::BlockingApiError {
                status_code,
                method: "".to_string(),
                url: url.to_string(),
                response,
            }),
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
        let authed_builder =
            self.builder_with_auth(unauthed_builder, &["ApiKeyAuth", "CookieAuth"]);
        let request_body: serde_json::Value =
            serde_json::to_value(request.data).map_err(result::Error::Serialize)?;
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
            400 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "POST".to_string(),
                    url: url.to_string(),
                    data: error_enums::CreateApiLinkErrors::Status400(data),
                })
            }
            401 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "POST".to_string(),
                    url: url.to_string(),
                    data: error_enums::CreateApiLinkErrors::Status401(data),
                })
            }
            404 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "POST".to_string(),
                    url: url.to_string(),
                    data: error_enums::CreateApiLinkErrors::Status404(data),
                })
            }
            500 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "POST".to_string(),
                    url: url.to_string(),
                    data: error_enums::CreateApiLinkErrors::Status500(data),
                })
            }
            _ => Err(result::Error::BlockingApiError {
                status_code,
                method: "".to_string(),
                url: url.to_string(),
                response,
            }),
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
        let authed_builder =
            self.builder_with_auth(unauthed_builder, &["ApiKeyAuth", "CookieAuth"]);
        let request_body: serde_json::Value =
            serde_json::to_value(request.data).map_err(result::Error::Serialize)?;
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
            400 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "POST".to_string(),
                    url: url.to_string(),
                    data: error_enums::ReorderApiLinksErrors::Status400(data),
                })
            }
            401 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "POST".to_string(),
                    url: url.to_string(),
                    data: error_enums::ReorderApiLinksErrors::Status401(data),
                })
            }
            404 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "POST".to_string(),
                    url: url.to_string(),
                    data: error_enums::ReorderApiLinksErrors::Status404(data),
                })
            }
            500 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "POST".to_string(),
                    url: url.to_string(),
                    data: error_enums::ReorderApiLinksErrors::Status500(data),
                })
            }
            _ => Err(result::Error::BlockingApiError {
                status_code,
                method: "".to_string(),
                url: url.to_string(),
                response,
            }),
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
        let authed_builder =
            self.builder_with_auth(unauthed_builder, &["ApiKeyAuth", "CookieAuth"]);
        let request_body: serde_json::Value =
            serde_json::to_value(request.data).map_err(result::Error::Serialize)?;
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
            400 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "POST".to_string(),
                    url: url.to_string(),
                    data: error_enums::CreateApiLinkGroupErrors::Status400(data),
                })
            }
            401 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "POST".to_string(),
                    url: url.to_string(),
                    data: error_enums::CreateApiLinkGroupErrors::Status401(data),
                })
            }
            404 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "POST".to_string(),
                    url: url.to_string(),
                    data: error_enums::CreateApiLinkGroupErrors::Status404(data),
                })
            }
            500 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "POST".to_string(),
                    url: url.to_string(),
                    data: error_enums::CreateApiLinkGroupErrors::Status500(data),
                })
            }
            _ => Err(result::Error::BlockingApiError {
                status_code,
                method: "".to_string(),
                url: url.to_string(),
                response,
            }),
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
        let authed_builder =
            self.builder_with_auth(unauthed_builder, &["ApiKeyAuth", "CookieAuth"]);
        let request_body: serde_json::Value =
            serde_json::to_value(request.data).map_err(result::Error::Serialize)?;
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
            400 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "POST".to_string(),
                    url: url.to_string(),
                    data: error_enums::CreateApiProjectErrors::Status400(data),
                })
            }
            401 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "POST".to_string(),
                    url: url.to_string(),
                    data: error_enums::CreateApiProjectErrors::Status401(data),
                })
            }
            404 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "POST".to_string(),
                    url: url.to_string(),
                    data: error_enums::CreateApiProjectErrors::Status404(data),
                })
            }
            500 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "POST".to_string(),
                    url: url.to_string(),
                    data: error_enums::CreateApiProjectErrors::Status500(data),
                })
            }
            _ => Err(result::Error::BlockingApiError {
                status_code,
                method: "".to_string(),
                url: url.to_string(),
                response,
            }),
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
        let authed_builder =
            self.builder_with_auth(unauthed_builder, &["ApiKeyAuth", "CookieAuth"]);
        let request_body: serde_json::Value =
            serde_json::to_value(request.data).map_err(result::Error::Serialize)?;
        let response = authed_builder
            .json(&request_body)
            .send()
            .map_err(result::Error::Dispatch)?;
        let status_code = response.status().as_u16();
        match status_code {
            201 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<serde_json::Value>(&response_text).unwrap();
                Ok(data)
            }
            400 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "POST".to_string(),
                    url: url.to_string(),
                    data: error_enums::GrantApiProjectRoleErrors::Status400(data),
                })
            }
            401 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "POST".to_string(),
                    url: url.to_string(),
                    data: error_enums::GrantApiProjectRoleErrors::Status401(data),
                })
            }
            404 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "POST".to_string(),
                    url: url.to_string(),
                    data: error_enums::GrantApiProjectRoleErrors::Status404(data),
                })
            }
            500 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "POST".to_string(),
                    url: url.to_string(),
                    data: error_enums::GrantApiProjectRoleErrors::Status500(data),
                })
            }
            _ => Err(result::Error::BlockingApiError {
                status_code,
                method: "".to_string(),
                url: url.to_string(),
                response,
            }),
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
        let authed_builder =
            self.builder_with_auth(unauthed_builder, &["ApiKeyAuth", "CookieAuth"]);
        let request_body: serde_json::Value =
            serde_json::to_value(request.data).map_err(result::Error::Serialize)?;
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
            400 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "POST".to_string(),
                    url: url.to_string(),
                    data: error_enums::CreateApiVersionErrors::Status400(data),
                })
            }
            401 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "POST".to_string(),
                    url: url.to_string(),
                    data: error_enums::CreateApiVersionErrors::Status401(data),
                })
            }
            404 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "POST".to_string(),
                    url: url.to_string(),
                    data: error_enums::CreateApiVersionErrors::Status404(data),
                })
            }
            500 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "POST".to_string(),
                    url: url.to_string(),
                    data: error_enums::CreateApiVersionErrors::Status500(data),
                })
            }
            _ => Err(result::Error::BlockingApiError {
                status_code,
                method: "".to_string(),
                url: url.to_string(),
                response,
            }),
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
        let authed_builder =
            self.builder_with_auth(unauthed_builder, &["ApiKeyAuth", "CookieAuth"]);
        let request_body: serde_json::Value =
            serde_json::to_value(request.data).map_err(result::Error::Serialize)?;
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
            400 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "POST".to_string(),
                    url: url.to_string(),
                    data: error_enums::CreateDocProjectErrors::Status400(data),
                })
            }
            401 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "POST".to_string(),
                    url: url.to_string(),
                    data: error_enums::CreateDocProjectErrors::Status401(data),
                })
            }
            404 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "POST".to_string(),
                    url: url.to_string(),
                    data: error_enums::CreateDocProjectErrors::Status404(data),
                })
            }
            500 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "POST".to_string(),
                    url: url.to_string(),
                    data: error_enums::CreateDocProjectErrors::Status500(data),
                })
            }
            _ => Err(result::Error::BlockingApiError {
                status_code,
                method: "".to_string(),
                url: url.to_string(),
                response,
            }),
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
        let authed_builder =
            self.builder_with_auth(unauthed_builder, &["ApiKeyAuth", "CookieAuth"]);
        let request_body: serde_json::Value =
            serde_json::to_value(request.data).map_err(result::Error::Serialize)?;
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
            400 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "POST".to_string(),
                    url: url.to_string(),
                    data: error_enums::TriggerDeploymentErrors::Status400(data),
                })
            }
            401 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "POST".to_string(),
                    url: url.to_string(),
                    data: error_enums::TriggerDeploymentErrors::Status401(data),
                })
            }
            404 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "POST".to_string(),
                    url: url.to_string(),
                    data: error_enums::TriggerDeploymentErrors::Status404(data),
                })
            }
            500 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "POST".to_string(),
                    url: url.to_string(),
                    data: error_enums::TriggerDeploymentErrors::Status500(data),
                })
            }
            _ => Err(result::Error::BlockingApiError {
                status_code,
                method: "".to_string(),
                url: url.to_string(),
                response,
            }),
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
        let authed_builder =
            self.builder_with_auth(unauthed_builder, &["ApiKeyAuth", "CookieAuth"]);
        let request_body: serde_json::Value =
            serde_json::to_value(request.data).map_err(result::Error::Serialize)?;
        let response = authed_builder
            .json(&request_body)
            .send()
            .map_err(result::Error::Dispatch)?;
        let status_code = response.status().as_u16();
        match status_code {
            201 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<serde_json::Value>(&response_text).unwrap();
                Ok(data)
            }
            400 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "POST".to_string(),
                    url: url.to_string(),
                    data: error_enums::GrantDocProjectRoleErrors::Status400(data),
                })
            }
            401 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "POST".to_string(),
                    url: url.to_string(),
                    data: error_enums::GrantDocProjectRoleErrors::Status401(data),
                })
            }
            404 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "POST".to_string(),
                    url: url.to_string(),
                    data: error_enums::GrantDocProjectRoleErrors::Status404(data),
                })
            }
            500 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "POST".to_string(),
                    url: url.to_string(),
                    data: error_enums::GrantDocProjectRoleErrors::Status500(data),
                })
            }
            _ => Err(result::Error::BlockingApiError {
                status_code,
                method: "".to_string(),
                url: url.to_string(),
                response,
            }),
        }
    }
    pub fn create_guide(
        &self,
        request: CreateGuideRequest,
    ) -> result::Result<Guide, error_enums::CreateGuideErrors> {
        let endpoint = format!(
            "/doc_project/{}/version/{}/guide",
            request.project_id_or_name, request.version_id
        );
        let url = format!("{}{}", self.base_url, endpoint);
        let query_params: Vec<(&str, String)> = vec![];
        let unauthed_builder = ReqwestClient::default().post(&url).query(&query_params);
        let authed_builder =
            self.builder_with_auth(unauthed_builder, &["ApiKeyAuth", "CookieAuth"]);
        let request_body: serde_json::Value =
            serde_json::to_value(request.data).map_err(result::Error::Serialize)?;
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
            400 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "POST".to_string(),
                    url: url.to_string(),
                    data: error_enums::CreateGuideErrors::Status400(data),
                })
            }
            401 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "POST".to_string(),
                    url: url.to_string(),
                    data: error_enums::CreateGuideErrors::Status401(data),
                })
            }
            404 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "POST".to_string(),
                    url: url.to_string(),
                    data: error_enums::CreateGuideErrors::Status404(data),
                })
            }
            500 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "POST".to_string(),
                    url: url.to_string(),
                    data: error_enums::CreateGuideErrors::Status500(data),
                })
            }
            _ => Err(result::Error::BlockingApiError {
                status_code,
                method: "".to_string(),
                url: url.to_string(),
                response,
            }),
        }
    }
    pub fn reorder_guides(
        &self,
        request: ReorderGuidesRequest,
    ) -> result::Result<Vec<GuideWithChildren>, error_enums::ReorderGuidesErrors> {
        let endpoint = format!(
            "/doc_project/{}/version/{}/guide/reorder",
            request.project_id_or_name, request.version_id
        );
        let url = format!("{}{}", self.base_url, endpoint);
        let query_params: Vec<(&str, String)> = vec![];
        let unauthed_builder = ReqwestClient::default().post(&url).query(&query_params);
        let authed_builder =
            self.builder_with_auth(unauthed_builder, &["ApiKeyAuth", "CookieAuth"]);
        let request_body: serde_json::Value =
            serde_json::to_value(request.data).map_err(result::Error::Serialize)?;
        let response = authed_builder
            .json(&request_body)
            .send()
            .map_err(result::Error::Dispatch)?;
        let status_code = response.status().as_u16();
        match status_code {
            200 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Vec<GuideWithChildren>>(&response_text).unwrap();
                Ok(data)
            }
            400 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "POST".to_string(),
                    url: url.to_string(),
                    data: error_enums::ReorderGuidesErrors::Status400(data),
                })
            }
            401 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "POST".to_string(),
                    url: url.to_string(),
                    data: error_enums::ReorderGuidesErrors::Status401(data),
                })
            }
            404 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "POST".to_string(),
                    url: url.to_string(),
                    data: error_enums::ReorderGuidesErrors::Status404(data),
                })
            }
            500 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "POST".to_string(),
                    url: url.to_string(),
                    data: error_enums::ReorderGuidesErrors::Status500(data),
                })
            }
            _ => Err(result::Error::BlockingApiError {
                status_code,
                method: "".to_string(),
                url: url.to_string(),
                response,
            }),
        }
    }
    pub fn create_organization(
        &self,
        request: CreateOrganizationRequest,
    ) -> result::Result<OrganizationWithRedirect, error_enums::CreateOrganizationErrors> {
        let endpoint = "/organization";
        let url = format!("{}{}", self.base_url, endpoint);
        let query_params: Vec<(&str, String)> = vec![];
        let unauthed_builder = ReqwestClient::default().post(&url).query(&query_params);
        let authed_builder =
            self.builder_with_auth(unauthed_builder, &["ApiKeyAuth", "CookieAuth"]);
        let request_body: serde_json::Value =
            serde_json::to_value(request.data).map_err(result::Error::Serialize)?;
        let response = authed_builder
            .json(&request_body)
            .send()
            .map_err(result::Error::Dispatch)?;
        let status_code = response.status().as_u16();
        match status_code {
            201 => {
                let response_text = response.text().unwrap_or_default();
                let data =
                    serde_json::from_str::<OrganizationWithRedirect>(&response_text).unwrap();
                Ok(data)
            }
            400 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "POST".to_string(),
                    url: url.to_string(),
                    data: error_enums::CreateOrganizationErrors::Status400(data),
                })
            }
            401 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "POST".to_string(),
                    url: url.to_string(),
                    data: error_enums::CreateOrganizationErrors::Status401(data),
                })
            }
            404 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "POST".to_string(),
                    url: url.to_string(),
                    data: error_enums::CreateOrganizationErrors::Status404(data),
                })
            }
            500 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "POST".to_string(),
                    url: url.to_string(),
                    data: error_enums::CreateOrganizationErrors::Status500(data),
                })
            }
            _ => Err(result::Error::BlockingApiError {
                status_code,
                method: "".to_string(),
                url: url.to_string(),
                response,
            }),
        }
    }
    /// Add a assets like logos or other media to an organization
    pub fn upload_assets(
        &self,
        request: UploadAssetsRequest,
    ) -> result::Result<Vec<Asset>, error_enums::UploadAssetsErrors> {
        let endpoint = "/organization/asset";
        let url = format!("{}{}", self.base_url, endpoint);
        let query_params: Vec<(&str, String)> = vec![];
        let unauthed_builder = ReqwestClient::default().post(&url).query(&query_params);
        let authed_builder =
            self.builder_with_auth(unauthed_builder, &["ApiKeyAuth", "CookieAuth"]);
        let mut form_data_body = reqwest_multipart::Form::new();
        form_data_body = form_data_body.part(
            "file",
            reqwest_multipart::Part::file(&request.data.file).map_err(result::Error::File)?,
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
            400 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "POST".to_string(),
                    url: url.to_string(),
                    data: error_enums::UploadAssetsErrors::Status400(data),
                })
            }
            401 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "POST".to_string(),
                    url: url.to_string(),
                    data: error_enums::UploadAssetsErrors::Status401(data),
                })
            }
            404 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "POST".to_string(),
                    url: url.to_string(),
                    data: error_enums::UploadAssetsErrors::Status404(data),
                })
            }
            500 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "POST".to_string(),
                    url: url.to_string(),
                    data: error_enums::UploadAssetsErrors::Status500(data),
                })
            }
            _ => Err(result::Error::BlockingApiError {
                status_code,
                method: "".to_string(),
                url: url.to_string(),
                response,
            }),
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
        let authed_builder =
            self.builder_with_auth(unauthed_builder, &["ApiKeyAuth", "CookieAuth"]);
        let request_body: serde_json::Value =
            serde_json::to_value(request.data).map_err(result::Error::Serialize)?;
        let response = authed_builder
            .json(&request_body)
            .send()
            .map_err(result::Error::Dispatch)?;
        let status_code = response.status().as_u16();
        match status_code {
            201 => {
                let res_bytes = response.bytes().map_err(result::Error::ResponseBytes)?;
                let data = BinaryResponse { content: res_bytes };
                Ok(data)
            }
            400 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "POST".to_string(),
                    url: url.to_string(),
                    data: error_enums::CreateSdkErrors::Status400(data),
                })
            }
            401 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "POST".to_string(),
                    url: url.to_string(),
                    data: error_enums::CreateSdkErrors::Status401(data),
                })
            }
            404 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "POST".to_string(),
                    url: url.to_string(),
                    data: error_enums::CreateSdkErrors::Status404(data),
                })
            }
            500 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "POST".to_string(),
                    url: url.to_string(),
                    data: error_enums::CreateSdkErrors::Status500(data),
                })
            }
            _ => Err(result::Error::BlockingApiError {
                status_code,
                method: "".to_string(),
                url: url.to_string(),
                response,
            }),
        }
    }
    pub fn update_sdk(
        &self,
        request: UpdateSdkRequest,
    ) -> result::Result<NewSdkResponse, error_enums::UpdateSdkErrors> {
        let endpoint = format!("/sdk/{}/{}", request.name, request.semver);
        let url = format!("{}{}", self.base_url, endpoint);
        let mut query_params: Vec<(&str, String)> = vec![];
        if let Some(api_version_id_or_semver) = request.api_version_id_or_semver {
            query_params.push((
                "api_version_id_or_semver",
                api_version_id_or_semver.to_string(),
            ));
        }
        let unauthed_builder = ReqwestClient::default().post(&url).query(&query_params);
        let authed_builder =
            self.builder_with_auth(unauthed_builder, &["ApiKeyAuth", "CookieAuth"]);
        let mut form_data_body = reqwest_multipart::Form::new();
        form_data_body = form_data_body.part(
            "file",
            reqwest_multipart::Part::file(&request.data.file).map_err(result::Error::File)?,
        );
        let response = authed_builder
            .multipart(form_data_body)
            .send()
            .map_err(result::Error::Dispatch)?;
        let status_code = response.status().as_u16();
        match status_code {
            200 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<NewSdkResponse>(&response_text).unwrap();
                Ok(data)
            }
            400 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "POST".to_string(),
                    url: url.to_string(),
                    data: error_enums::UpdateSdkErrors::Status400(data),
                })
            }
            401 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "POST".to_string(),
                    url: url.to_string(),
                    data: error_enums::UpdateSdkErrors::Status401(data),
                })
            }
            404 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "POST".to_string(),
                    url: url.to_string(),
                    data: error_enums::UpdateSdkErrors::Status404(data),
                })
            }
            500 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "POST".to_string(),
                    url: url.to_string(),
                    data: error_enums::UpdateSdkErrors::Status500(data),
                })
            }
            _ => Err(result::Error::BlockingApiError {
                status_code,
                method: "".to_string(),
                url: url.to_string(),
                response,
            }),
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
        let authed_builder =
            self.builder_with_auth(unauthed_builder, &["ApiKeyAuth", "CookieAuth"]);
        let request_body: serde_json::Value =
            serde_json::to_value(request.data).map_err(result::Error::Serialize)?;
        let response = authed_builder
            .json(&request_body)
            .send()
            .map_err(result::Error::Dispatch)?;
        let status_code = response.status().as_u16();
        match status_code {
            201 => {
                let res_bytes = response.bytes().map_err(result::Error::ResponseBytes)?;
                let data = BinaryResponse { content: res_bytes };
                Ok(data)
            }
            400 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "POST".to_string(),
                    url: url.to_string(),
                    data: error_enums::StatelessGenerateSdkErrors::Status400(data),
                })
            }
            401 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "POST".to_string(),
                    url: url.to_string(),
                    data: error_enums::StatelessGenerateSdkErrors::Status401(data),
                })
            }
            404 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "POST".to_string(),
                    url: url.to_string(),
                    data: error_enums::StatelessGenerateSdkErrors::Status404(data),
                })
            }
            500 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "POST".to_string(),
                    url: url.to_string(),
                    data: error_enums::StatelessGenerateSdkErrors::Status500(data),
                })
            }
            _ => Err(result::Error::BlockingApiError {
                status_code,
                method: "".to_string(),
                url: url.to_string(),
                response,
            }),
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
        let authed_builder =
            self.builder_with_auth(unauthed_builder, &["ApiKeyAuth", "CookieAuth"]);
        let request_body: serde_json::Value =
            serde_json::to_value(request.data).map_err(result::Error::Serialize)?;
        let response = authed_builder
            .json(&request_body)
            .send()
            .map_err(result::Error::Dispatch)?;
        let status_code = response.status().as_u16();
        match status_code {
            201 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<serde_json::Value>(&response_text).unwrap();
                Ok(data)
            }
            400 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "POST".to_string(),
                    url: url.to_string(),
                    data: error_enums::InviteUserErrors::Status400(data),
                })
            }
            401 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "POST".to_string(),
                    url: url.to_string(),
                    data: error_enums::InviteUserErrors::Status401(data),
                })
            }
            404 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "POST".to_string(),
                    url: url.to_string(),
                    data: error_enums::InviteUserErrors::Status404(data),
                })
            }
            500 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "POST".to_string(),
                    url: url.to_string(),
                    data: error_enums::InviteUserErrors::Status500(data),
                })
            }
            _ => Err(result::Error::BlockingApiError {
                status_code,
                method: "".to_string(),
                url: url.to_string(),
                response,
            }),
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
        let authed_builder =
            self.builder_with_auth(unauthed_builder, &["ApiKeyAuth", "CookieAuth"]);
        let request_body: serde_json::Value =
            serde_json::to_value(request.data).map_err(result::Error::Serialize)?;
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
            400 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "POST".to_string(),
                    url: url.to_string(),
                    data: error_enums::CreateServiceAccountErrors::Status400(data),
                })
            }
            401 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "POST".to_string(),
                    url: url.to_string(),
                    data: error_enums::CreateServiceAccountErrors::Status401(data),
                })
            }
            404 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "POST".to_string(),
                    url: url.to_string(),
                    data: error_enums::CreateServiceAccountErrors::Status404(data),
                })
            }
            500 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "POST".to_string(),
                    url: url.to_string(),
                    data: error_enums::CreateServiceAccountErrors::Status500(data),
                })
            }
            _ => Err(result::Error::BlockingApiError {
                status_code,
                method: "".to_string(),
                url: url.to_string(),
                response,
            }),
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
        let authed_builder =
            self.builder_with_auth(unauthed_builder, &["ApiKeyAuth", "CookieAuth"]);
        let request_body: serde_json::Value =
            serde_json::to_value(request.data).map_err(result::Error::Serialize)?;
        let response = authed_builder
            .json(&request_body)
            .send()
            .map_err(result::Error::Dispatch)?;
        let status_code = response.status().as_u16();
        match status_code {
            202 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<serde_json::Value>(&response_text).unwrap();
                Ok(data)
            }
            _ => Err(result::Error::BlockingApiError {
                status_code,
                method: "".to_string(),
                url: url.to_string(),
                response,
            }),
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
        let authed_builder =
            self.builder_with_auth(unauthed_builder, &["ApiKeyAuth", "CookieAuth"]);
        let request_body: serde_json::Value =
            serde_json::to_value(request.data).map_err(result::Error::Serialize)?;
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
            400 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "PUT".to_string(),
                    url: url.to_string(),
                    data: error_enums::UpdateDocProjectThemeErrors::Status400(data),
                })
            }
            401 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "PUT".to_string(),
                    url: url.to_string(),
                    data: error_enums::UpdateDocProjectThemeErrors::Status401(data),
                })
            }
            404 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "PUT".to_string(),
                    url: url.to_string(),
                    data: error_enums::UpdateDocProjectThemeErrors::Status404(data),
                })
            }
            500 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "PUT".to_string(),
                    url: url.to_string(),
                    data: error_enums::UpdateDocProjectThemeErrors::Status500(data),
                })
            }
            _ => Err(result::Error::BlockingApiError {
                status_code,
                method: "".to_string(),
                url: url.to_string(),
                response,
            }),
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
        let authed_builder =
            self.builder_with_auth(unauthed_builder, &["ApiKeyAuth", "CookieAuth"]);
        let request_body: serde_json::Value =
            serde_json::to_value(request.data).map_err(result::Error::Serialize)?;
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
            400 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "PUT".to_string(),
                    url: url.to_string(),
                    data: error_enums::UpdateOrganizationThemeErrors::Status400(data),
                })
            }
            401 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "PUT".to_string(),
                    url: url.to_string(),
                    data: error_enums::UpdateOrganizationThemeErrors::Status401(data),
                })
            }
            404 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "PUT".to_string(),
                    url: url.to_string(),
                    data: error_enums::UpdateOrganizationThemeErrors::Status404(data),
                })
            }
            500 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "PUT".to_string(),
                    url: url.to_string(),
                    data: error_enums::UpdateOrganizationThemeErrors::Status500(data),
                })
            }
            _ => Err(result::Error::BlockingApiError {
                status_code,
                method: "".to_string(),
                url: url.to_string(),
                response,
            }),
        }
    }
}