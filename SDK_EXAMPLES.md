
## SDK Usage Examples


### Delete a API specification collection


**API Endpoint**: `DELETE /api/{id}`


#### Example Snippet

```rust
let client = sideko_rest_api::Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").unwrap())
    .with_cookie_auth(&std::env::var("API_KEY").unwrap());
let res = client
    .api()
    .delete(sideko_rest_api::resources::api::DeleteRequest {
        id: "string".to_string(),
    })
    .await;
```

    
### Remove role for a user for an API specification collection


**API Endpoint**: `DELETE /api/{id}/role/{user_id}`


#### Example Snippet

```rust
let client = sideko_rest_api::Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").unwrap())
    .with_cookie_auth(&std::env::var("API_KEY").unwrap());
let res = client
    .api()
    .role()
    .delete(sideko_rest_api::resources::api::role::DeleteRequest {
        id: "string".to_string(),
        user_id: "string".to_string(),
    })
    .await;
```

    
### Removes an API link


**API Endpoint**: `DELETE /api_link/{link_id}`


#### Example Snippet

```rust
let client = sideko_rest_api::Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").unwrap())
    .with_cookie_auth(&std::env::var("API_KEY").unwrap());
let res = client
    .delete_api_link(sideko_rest_api::DeleteApiLinkRequest {
        link_id: "3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a".to_string(),
    })
    .await;
```

    
### Deletes the api group and all its links


**API Endpoint**: `DELETE /api_link_group/{group_id}`


#### Example Snippet

```rust
let client = sideko_rest_api::Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").unwrap())
    .with_cookie_auth(&std::env::var("API_KEY").unwrap());
let res = client
    .delete_api_link_group(sideko_rest_api::DeleteApiLinkGroupRequest {
        group_id: "3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a".to_string(),
    })
    .await;
```

    
### Delete a specific Documentation Project


**API Endpoint**: `DELETE /doc_project/{project_id_or_name}`


#### Example Snippet

```rust
let client = sideko_rest_api::Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").unwrap())
    .with_cookie_auth(&std::env::var("API_KEY").unwrap());
let res = client
    .delete_doc_project(sideko_rest_api::DeleteDocProjectRequest {
        project_id_or_name: "string".to_string(),
    })
    .await;
```

    
### Remove role for a user for a doc project.


**API Endpoint**: `DELETE /doc_project/{project_id_or_name}/role/{user_id}`


#### Example Snippet

```rust
let client = sideko_rest_api::Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").unwrap())
    .with_cookie_auth(&std::env::var("API_KEY").unwrap());
let res = client
    .delete_doc_project_role(sideko_rest_api::DeleteDocProjectRoleRequest {
        project_id_or_name: "string".to_string(),
        user_id: "string".to_string(),
    })
    .await;
```

    
### Delete a specific guide for a specific version of a documentation project


**API Endpoint**: `DELETE /doc_project/{project_id_or_name}/version/{version_id}/guide/{guide_id}`


#### Example Snippet

```rust
let client = sideko_rest_api::Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").unwrap())
    .with_cookie_auth(&std::env::var("API_KEY").unwrap());
let res = client
    .delete_guide(sideko_rest_api::DeleteGuideRequest {
        project_id_or_name: "string".to_string(),
        version_id: "string".to_string(),
        guide_id: "string".to_string(),
    })
    .await;
```

    
### Delete href for a specific guide for a specific version of a documentation project


**API Endpoint**: `DELETE /doc_project/{project_id_or_name}/version/{version_id}/guide/{guide_id}/href`


#### Example Snippet

```rust
let client = sideko_rest_api::Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").unwrap())
    .with_cookie_auth(&std::env::var("API_KEY").unwrap());
let res = client
    .delete_guide_href(sideko_rest_api::DeleteGuideHrefRequest {
        project_id_or_name: "string".to_string(),
        version_id: "string".to_string(),
        guide_id: "string".to_string(),
        variant: sideko_rest_api::models::GuideHrefVariantEnum::Next,
    })
    .await;
```

    
### 
Delete an asset in your organization

**API Endpoint**: `DELETE /organization/asset/{asset_id}`


#### Example Snippet

```rust
let client = sideko_rest_api::Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").unwrap())
    .with_cookie_auth(&std::env::var("API_KEY").unwrap());
let res = client
    .delete_asset(sideko_rest_api::DeleteAssetRequest {
        asset_id: "3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a".to_string(),
    })
    .await;
```

    
### Delete a service account


**API Endpoint**: `DELETE /user/service_account/{service_account_id}`


#### Example Snippet

```rust
let client = sideko_rest_api::Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").unwrap())
    .with_cookie_auth(&std::env::var("API_KEY").unwrap());
let res = client
    .delete_service_account(sideko_rest_api::DeleteServiceAccountRequest {
        service_account_id: "3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a".to_string(),
    })
    .await;
```

    
### Health Check


**API Endpoint**: `GET /_health`


#### Example Snippet

```rust
let client = sideko_rest_api::Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").unwrap())
    .with_cookie_auth(&std::env::var("API_KEY").unwrap());
let res = client.health_check().await;
```

    
### Ping Check


**API Endpoint**: `GET /_ping`


#### Example Snippet

```rust
let client = sideko_rest_api::Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").unwrap())
    .with_cookie_auth(&std::env::var("API_KEY").unwrap());
let res = client.ping_check().await;
```

    
### List your API specification collections


**API Endpoint**: `GET /api`


#### Example Snippet

```rust
let client = sideko_rest_api::Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").unwrap())
    .with_cookie_auth(&std::env::var("API_KEY").unwrap());
let res = client.api().list().await;
```

    
### Get a API Specification collection


**API Endpoint**: `GET /api/{id}`


#### Example Snippet

```rust
let client = sideko_rest_api::Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").unwrap())
    .with_cookie_auth(&std::env::var("API_KEY").unwrap());
let res = client
    .api()
    .get(sideko_rest_api::resources::api::GetRequest {
        id: "string".to_string(),
    })
    .await;
```

    
### List the members of the API specification collection


**API Endpoint**: `GET /api/{id}/members`


#### Example Snippet

```rust
let client = sideko_rest_api::Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").unwrap())
    .with_cookie_auth(&std::env::var("API_KEY").unwrap());
let res = client
    .api()
    .members()
    .list(sideko_rest_api::resources::api::members::ListRequest {
        id: "string".to_string(),
    })
    .await;
```

    
### List specs of a collection


**API Endpoint**: `GET /api/{id}/spec`


#### Example Snippet

```rust
let client = sideko_rest_api::Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").unwrap())
    .with_cookie_auth(&std::env::var("API_KEY").unwrap());
let res = client
    .api()
    .spec()
    .list(sideko_rest_api::resources::api::spec::ListRequest {
        id: "string".to_string(),
    })
    .await;
```

    
### Get API specification metadata


**API Endpoint**: `GET /api/{id}/spec/{version}`


#### Example Snippet

```rust
let client = sideko_rest_api::Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").unwrap())
    .with_cookie_auth(&std::env::var("API_KEY").unwrap());
let res = client
    .api()
    .spec()
    .get(sideko_rest_api::resources::api::spec::GetRequest {
        id: "string".to_string(),
        version: "string".to_string(),
    })
    .await;
```

    
### Get OpenAPI specification


**API Endpoint**: `GET /api/{id}/spec/{version}/openapi`


#### Example Snippet

```rust
let client = sideko_rest_api::Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").unwrap())
    .with_cookie_auth(&std::env::var("API_KEY").unwrap());
let res = client
    .api()
    .spec()
    .openapi()
    .list(sideko_rest_api::resources::api::spec::openapi::ListRequest {
        id: "string".to_string(),
        version: "string".to_string(),
    })
    .await;
```

    
### Get Stats about the specification


**API Endpoint**: `GET /api/{id}/spec/{version}/stats`


#### Example Snippet

```rust
let client = sideko_rest_api::Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").unwrap())
    .with_cookie_auth(&std::env::var("API_KEY").unwrap());
let res = client
    .get_api_version_stats(sideko_rest_api::GetApiVersionStatsRequest {
        id: "string".to_string(),
        version: "string".to_string(),
    })
    .await;
```

    
### List API links for doc version


**API Endpoint**: `GET /api_link`


#### Example Snippet

```rust
let client = sideko_rest_api::Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").unwrap())
    .with_cookie_auth(&std::env::var("API_KEY").unwrap());
let res = client
    .list_api_links(sideko_rest_api::ListApiLinksRequest {
        doc_version_id: "3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a".to_string(),
    })
    .await;
```

    
### Retrieve single API link


**API Endpoint**: `GET /api_link/{link_id}`


#### Example Snippet

```rust
let client = sideko_rest_api::Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").unwrap())
    .with_cookie_auth(&std::env::var("API_KEY").unwrap());
let res = client
    .get_api_link(sideko_rest_api::GetApiLinkRequest {
        link_id: "3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a".to_string(),
    })
    .await;
```

    
### List API groups for doc version


**API Endpoint**: `GET /api_link_group`


#### Example Snippet

```rust
let client = sideko_rest_api::Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").unwrap())
    .with_cookie_auth(&std::env::var("API_KEY").unwrap());
let res = client
    .list_api_link_groups(sideko_rest_api::ListApiLinkGroupsRequest {
        doc_version_id: "3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a".to_string(),
    })
    .await;
```

    
### Exchange one-time auth key for api key


**API Endpoint**: `GET /auth/exchange_key`


#### Example Snippet

```rust
let client = sideko_rest_api::Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").unwrap())
    .with_cookie_auth(&std::env::var("API_KEY").unwrap());
let res = client
    .exchange_code_for_key(sideko_rest_api::ExchangeCodeForKeyRequest {
        code: "string".to_string(),
    })
    .await;
```

    
### Callback for handling authentication response


**API Endpoint**: `GET /auth/login_callback`


#### Example Snippet

```rust
let client = sideko_rest_api::Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").unwrap())
    .with_cookie_auth(&std::env::var("API_KEY").unwrap());
let res = client
    .login_callback(sideko_rest_api::LoginCallbackRequest {
        code: "string".to_string(),
        ..Default::default()
    })
    .await;
```

    
### Get URL for initiating login process


**API Endpoint**: `GET /auth/login_url`


#### Example Snippet

```rust
let client = sideko_rest_api::Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").unwrap())
    .with_cookie_auth(&std::env::var("API_KEY").unwrap());
let res = client
    .login_url(sideko_rest_api::LoginUrlRequest {
        ..Default::default()
    })
    .await;
```

    
### Checks if current CLI has updates


**API Endpoint**: `GET /cli/updates/{cli_version}`


#### Example Snippet

```rust
let client = sideko_rest_api::Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").unwrap())
    .with_cookie_auth(&std::env::var("API_KEY").unwrap());
let res = client
    .cli_check_updates(sideko_rest_api::CliCheckUpdatesRequest {
        cli_version: "0.1.0".to_string(),
    })
    .await;
```

    
### List Documentation Projects


**API Endpoint**: `GET /doc_project`


#### Example Snippet

```rust
let client = sideko_rest_api::Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").unwrap())
    .with_cookie_auth(&std::env::var("API_KEY").unwrap());
let res = client.list_doc_projects().await;
```

    
### Get a specific Documentation Project


**API Endpoint**: `GET /doc_project/{project_id_or_name}`


#### Example Snippet

```rust
let client = sideko_rest_api::Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").unwrap())
    .with_cookie_auth(&std::env::var("API_KEY").unwrap());
let res = client
    .get_doc_project(sideko_rest_api::GetDocProjectRequest {
        project_id_or_name: "string".to_string(),
    })
    .await;
```

    
### List deployments for a specific documentation project
Retrieves all deployments for a doc project

**API Endpoint**: `GET /doc_project/{project_id_or_name}/deployment`


#### Example Snippet

```rust
let client = sideko_rest_api::Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").unwrap())
    .with_cookie_auth(&std::env::var("API_KEY").unwrap());
let res = client
    .list_deployments(sideko_rest_api::ListDeploymentsRequest {
        project_id_or_name: "string".to_string(),
        ..Default::default()
    })
    .await;
```

    
### Get a specific deployment for a specific documentation project
Retrieves single deployment

**API Endpoint**: `GET /doc_project/{project_id_or_name}/deployment/{deployment_id}`


#### Example Snippet

```rust
let client = sideko_rest_api::Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").unwrap())
    .with_cookie_auth(&std::env::var("API_KEY").unwrap());
let res = client
    .get_deployment(sideko_rest_api::GetDeploymentRequest {
        project_id_or_name: "string".to_string(),
        deployment_id: "3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a".to_string(),
    })
    .await;
```

    
### List the members of the Doc Project


**API Endpoint**: `GET /doc_project/{project_id_or_name}/members`


#### Example Snippet

```rust
let client = sideko_rest_api::Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").unwrap())
    .with_cookie_auth(&std::env::var("API_KEY").unwrap());
let res = client
    .list_doc_project_members(sideko_rest_api::ListDocProjectMembersRequest {
        project_id_or_name: "string".to_string(),
    })
    .await;
```

    
### A simple check to see if the requesting user has access to the preview doc project


**API Endpoint**: `GET /doc_project/{project_id_or_name}/preview`


#### Example Snippet

```rust
let client = sideko_rest_api::Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").unwrap())
    .with_cookie_auth(&std::env::var("API_KEY").unwrap());
let res = client
    .check_preview(sideko_rest_api::CheckPreviewRequest {
        project_id_or_name: "string".to_string(),
    })
    .await;
```

    
### Get the theme attached to a documentation project


**API Endpoint**: `GET /doc_project/{project_id_or_name}/theme`


#### Example Snippet

```rust
let client = sideko_rest_api::Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").unwrap())
    .with_cookie_auth(&std::env::var("API_KEY").unwrap());
let res = client
    .get_doc_project_theme(sideko_rest_api::GetDocProjectThemeRequest {
        project_id_or_name: "string".to_string(),
    })
    .await;
```

    
### List versions of a specific Documentation Project


**API Endpoint**: `GET /doc_project/{project_id_or_name}/version`


#### Example Snippet

```rust
let client = sideko_rest_api::Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").unwrap())
    .with_cookie_auth(&std::env::var("API_KEY").unwrap());
let res = client
    .list_doc_versions(sideko_rest_api::ListDocVersionsRequest {
        project_id_or_name: "string".to_string(),
    })
    .await;
```

    
### Get a specific version of an Documentation Project


**API Endpoint**: `GET /doc_project/{project_id_or_name}/version/{version_id}`


#### Example Snippet

```rust
let client = sideko_rest_api::Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").unwrap())
    .with_cookie_auth(&std::env::var("API_KEY").unwrap());
let res = client
    .get_doc_version(sideko_rest_api::GetDocVersionRequest {
        project_id_or_name: "string".to_string(),
        version_id: "string".to_string(),
    })
    .await;
```

    
### List guides for a specific version of a documentation project


**API Endpoint**: `GET /doc_project/{project_id_or_name}/version/{version_id}/guide`


#### Example Snippet

```rust
let client = sideko_rest_api::Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").unwrap())
    .with_cookie_auth(&std::env::var("API_KEY").unwrap());
let res = client
    .list_guides(sideko_rest_api::ListGuidesRequest {
        project_id_or_name: "string".to_string(),
        version_id: "string".to_string(),
    })
    .await;
```

    
### Get a specific guide for a specific version of a documentation project


**API Endpoint**: `GET /doc_project/{project_id_or_name}/version/{version_id}/guide/{guide_id}`


#### Example Snippet

```rust
let client = sideko_rest_api::Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").unwrap())
    .with_cookie_auth(&std::env::var("API_KEY").unwrap());
let res = client
    .get_guide(sideko_rest_api::GetGuideRequest {
        project_id_or_name: "string".to_string(),
        version_id: "string".to_string(),
        guide_id: "string".to_string(),
    })
    .await;
```

    
### Get content for a specific guide for a specific version of a documentation project


**API Endpoint**: `GET /doc_project/{project_id_or_name}/version/{version_id}/guide/{guide_id}/content`


#### Example Snippet

```rust
let client = sideko_rest_api::Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").unwrap())
    .with_cookie_auth(&std::env::var("API_KEY").unwrap());
let res = client
    .get_guide_content(sideko_rest_api::GetGuideContentRequest {
        project_id_or_name: "string".to_string(),
        version_id: "string".to_string(),
        guide_id: "string".to_string(),
    })
    .await;
```

    
### 
Get user organization

**API Endpoint**: `GET /organization`


#### Example Snippet

```rust
let client = sideko_rest_api::Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").unwrap())
    .with_cookie_auth(&std::env::var("API_KEY").unwrap());
let res = client.get_organization().await;
```

    
### 
Get all assets for an organization

**API Endpoint**: `GET /organization/asset`


#### Example Snippet

```rust
let client = sideko_rest_api::Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").unwrap())
    .with_cookie_auth(&std::env::var("API_KEY").unwrap());
let res = client
    .list_assets(sideko_rest_api::ListAssetsRequest {
        ..Default::default()
    })
    .await;
```

    
### 
Get users in the organization

**API Endpoint**: `GET /organization/members`


#### Example Snippet

```rust
let client = sideko_rest_api::Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").unwrap())
    .with_cookie_auth(&std::env::var("API_KEY").unwrap());
let res = client.list_organization_members().await;
```

    
### 
Get documentation project theme configured at the organization level

**API Endpoint**: `GET /organization/theme`


#### Example Snippet

```rust
let client = sideko_rest_api::Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").unwrap())
    .with_cookie_auth(&std::env::var("API_KEY").unwrap());
let res = client.get_organization_theme().await;
```

    
### List all SDKs for an API Specification


**API Endpoint**: `GET /sdk/{api_id}`


#### Example Snippet

```rust
let client = sideko_rest_api::Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").unwrap())
    .with_cookie_auth(&std::env::var("API_KEY").unwrap());
let res = client
    .list_sdks(sideko_rest_api::ListSdksRequest {
        api_id: "string".to_string(),
    })
    .await;
```

    
### Get current user profile


**API Endpoint**: `GET /user/me`


#### Example Snippet

```rust
let client = sideko_rest_api::Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").unwrap())
    .with_cookie_auth(&std::env::var("API_KEY").unwrap());
let res = client.get_current_user().await;
```

    
### Get API key for the current user


**API Endpoint**: `GET /user/me/api_key`


#### Example Snippet

```rust
let client = sideko_rest_api::Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").unwrap())
    .with_cookie_auth(&std::env::var("API_KEY").unwrap());
let res = client.get_api_key().await;
```

    
### 
retrieve current user role for a given project type/id

**API Endpoint**: `GET /user/me/project_role`


#### Example Snippet

```rust
let client = sideko_rest_api::Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").unwrap())
    .with_cookie_auth(&std::env::var("API_KEY").unwrap());
let res = client
    .get_user_project_role(sideko_rest_api::GetUserProjectRoleRequest {
        project_type: sideko_rest_api::models::ProjectTypeEnum::Api,
        ..Default::default()
    })
    .await;
```

    
### Get all of your Service Accounts


**API Endpoint**: `GET /user/service_account`


#### Example Snippet

```rust
let client = sideko_rest_api::Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").unwrap())
    .with_cookie_auth(&std::env::var("API_KEY").unwrap());
let res = client.get_service_accounts().await;
```

    
### Update an existing API Specification collection


**API Endpoint**: `PATCH /api/{id}`


#### Example Snippet

```rust
let client = sideko_rest_api::Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").unwrap())
    .with_cookie_auth(&std::env::var("API_KEY").unwrap());
let res = client
    .api()
    .patch(sideko_rest_api::resources::api::PatchRequest {
        id: "string".to_string(),
        data: sideko_rest_api::models::UpdateApi {
            id: Some("my-new-api-name".to_string()),
        },
    })
    .await;
```

    
### Update an API Specification and/or metadata


**API Endpoint**: `PATCH /api/{id}/spec/{version}`


#### Example Snippet

```rust
let client = sideko_rest_api::Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").unwrap())
    .with_cookie_auth(&std::env::var("API_KEY").unwrap());
let res = client
    .api()
    .spec()
    .patch(sideko_rest_api::resources::api::spec::PatchRequest {
        id: "string".to_string(),
        version: "string".to_string(),
        data: sideko_rest_api::models::UpdateApiSpec {
            mock_server_enabled: Some(true),
            notes: Some(
                "<p>This version includes a number of excellent improvements</p>"
                    .to_string(),
            ),
            openapi: Some(
                sideko_rest_api::UploadFile::from_path("tests/file.pdf").unwrap(),
            ),
            semver: Some("string".to_string()),
        },
    })
    .await;
```

    
### Updates an API link


**API Endpoint**: `PATCH /api_link/{link_id}`


#### Example Snippet

```rust
let client = sideko_rest_api::Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").unwrap())
    .with_cookie_auth(&std::env::var("API_KEY").unwrap());
let res = client
    .update_api_link(sideko_rest_api::UpdateApiLinkRequest {
        link_id: "3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a".to_string(),
        data: sideko_rest_api::models::UpdateApiLink {
            api_version: Some("string".to_string()),
            build_request_enabled: Some(true),
            include_mock_server: Some(true),
            nav_label: Some("string".to_string()),
            policy: Some(sideko_rest_api::models::ApiLinkPolicyEnum::Latest),
            slug: Some("string".to_string()),
        },
    })
    .await;
```

    
### Updates API link group


**API Endpoint**: `PATCH /api_link_group/{group_id}`


#### Example Snippet

```rust
let client = sideko_rest_api::Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").unwrap())
    .with_cookie_auth(&std::env::var("API_KEY").unwrap());
let res = client
    .update_api_link_group(sideko_rest_api::UpdateApiLinkGroupRequest {
        group_id: "3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a".to_string(),
        data: sideko_rest_api::models::UpdateApiLinkGroup {
            nav_label: Some("string".to_string()),
            slug: Some("string".to_string()),
        },
    })
    .await;
```

    
### Update an existing Documentation Project


**API Endpoint**: `PATCH /doc_project/{project_id_or_name}`


#### Example Snippet

```rust
let client = sideko_rest_api::Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").unwrap())
    .with_cookie_auth(&std::env::var("API_KEY").unwrap());
let res = client
    .update_doc_project(sideko_rest_api::UpdateDocProjectRequest {
        project_id_or_name: "string".to_string(),
        data: sideko_rest_api::models::UpdateDocProject {
            logos: Some(sideko_rest_api::models::UpdateDocProjectLogos {
                dark: Some("3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a".to_string()),
                favicon: Some("3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a".to_string()),
                light: Some("3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a".to_string()),
            }),
            settings: Some(sideko_rest_api::models::UpdateDocProjectSettings {
                action_button: Some(sideko_rest_api::models::UpdateDocProjectSettingsActionButton {
                    enabled: Some(true),
                    label: Some("string".to_string()),
                    url: Some("http://www.example.com".to_string()),
                }),
                metadata: Some(sideko_rest_api::models::UpdateDocProjectSettingsMetadata {
                    description: Some("string".to_string()),
                    title: Some("string".to_string()),
                }),
            }),
            title: Some("my-company-docs".to_string()),
        },
    })
    .await;
```

    
### Update a specific guide for a specific version of a documentation project


**API Endpoint**: `PATCH /doc_project/{project_id_or_name}/version/{version_id}/guide/{guide_id}`


#### Example Snippet

```rust
let client = sideko_rest_api::Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").unwrap())
    .with_cookie_auth(&std::env::var("API_KEY").unwrap());
let res = client
    .update_guide(sideko_rest_api::UpdateGuideRequest {
        project_id_or_name: "string".to_string(),
        version_id: "string".to_string(),
        guide_id: "string".to_string(),
        data: sideko_rest_api::models::UpdateGuide {
            content: Some("string".to_string()),
            nav_label: Some("string".to_string()),
            next_id: Some("3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a".to_string()),
            prev_id: Some("3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a".to_string()),
            slug: Some("string".to_string()),
        },
    })
    .await;
```

    
### 
Update an asset in your organization

**API Endpoint**: `PATCH /organization/asset/{asset_id}`


#### Example Snippet

```rust
let client = sideko_rest_api::Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").unwrap())
    .with_cookie_auth(&std::env::var("API_KEY").unwrap());
let res = client
    .update_asset(sideko_rest_api::UpdateAssetRequest {
        asset_id: "3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a".to_string(),
        data: sideko_rest_api::models::UpdateAsset {
            name: Some("string".to_string()),
        },
    })
    .await;
```

    
### Create new API specification collection


**API Endpoint**: `POST /api`


#### Example Snippet

```rust
let client = sideko_rest_api::Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").unwrap())
    .with_cookie_auth(&std::env::var("API_KEY").unwrap());
let res = client
    .api()
    .create(sideko_rest_api::resources::api::CreateRequest {
        data: sideko_rest_api::models::NewApi {
            id: "my-api-spec".to_string(),
        },
    })
    .await;
```

    
### Grant user role for an API specification collection


**API Endpoint**: `POST /api/{id}/role`


#### Example Snippet

```rust
let client = sideko_rest_api::Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").unwrap())
    .with_cookie_auth(&std::env::var("API_KEY").unwrap());
let res = client
    .api()
    .role()
    .create(sideko_rest_api::resources::api::role::CreateRequest {
        id: "string".to_string(),
        data: sideko_rest_api::models::NewProjectRole {
            role: sideko_rest_api::models::ProjectRoleEnum::Admin,
            user_id: "3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a".to_string(),
        },
    })
    .await;
```

    
### Add a new API specification


**API Endpoint**: `POST /api/{id}/spec`


#### Example Snippet

```rust
let client = sideko_rest_api::Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").unwrap())
    .with_cookie_auth(&std::env::var("API_KEY").unwrap());
let res = client
    .api()
    .spec()
    .create(sideko_rest_api::resources::api::spec::CreateRequest {
        id: "string".to_string(),
        data: sideko_rest_api::models::NewApiSpec {
            mock_server_enabled: Some(true),
            notes: Some(
                "<p>This version includes a number of excellent improvements</p>"
                    .to_string(),
            ),
            openapi: sideko_rest_api::UploadFile::from_path("tests/file.pdf")
                .unwrap(),
            version: "string".to_string(),
        },
    })
    .await;
```

    
### Links API Version to Documentation project version with a specified update policy


**API Endpoint**: `POST /api_link`


#### Example Snippet

```rust
let client = sideko_rest_api::Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").unwrap())
    .with_cookie_auth(&std::env::var("API_KEY").unwrap());
let res = client
    .create_api_link(sideko_rest_api::CreateApiLinkRequest {
        data: sideko_rest_api::models::NewApiLink {
            build_request_enabled: Some(true),
            doc_version_id: "3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a".to_string(),
            group_id: "3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a".to_string(),
            include_mock_server: Some(true),
            nav_label: "string".to_string(),
            policy: sideko_rest_api::models::NewApiLinkPolicy::LatestApiLinkPolicy(sideko_rest_api::models::LatestApiLinkPolicy {
                api_id: "my-api".to_string(),
                type_field: sideko_rest_api::models::LatestApiLinkPolicyTypeEnum::Latest,
            }),
            slug: "string".to_string(),
        },
    })
    .await;
```

    
### Reorder API links and groups


**API Endpoint**: `POST /api_link/reorder`


#### Example Snippet

```rust
let client = sideko_rest_api::Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").unwrap())
    .with_cookie_auth(&std::env::var("API_KEY").unwrap());
let res = client
    .reorder_api_links(sideko_rest_api::ReorderApiLinksRequest {
        data: sideko_rest_api::models::ApiReorder {
            doc_version_id: "3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a".to_string(),
            groups: vec![
                sideko_rest_api::models::ApiLinkGroupReorder { id :
                "3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a".to_string(), order : 123 }
            ],
            links: vec![
                sideko_rest_api::models::ApiLinkReorder { group_id :
                "3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a".to_string(), id :
                "3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a".to_string(), order : 123 }
            ],
        },
    })
    .await;
```

    
### Create API group to organize API links


**API Endpoint**: `POST /api_link_group`


#### Example Snippet

```rust
let client = sideko_rest_api::Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").unwrap())
    .with_cookie_auth(&std::env::var("API_KEY").unwrap());
let res = client
    .create_api_link_group(sideko_rest_api::CreateApiLinkGroupRequest {
        data: sideko_rest_api::models::NewApiLinkGroup {
            doc_version_id: "3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a".to_string(),
            nav_label: "string".to_string(),
            slug: "string".to_string(),
        },
    })
    .await;
```

    
### Create a new Documentation Project


**API Endpoint**: `POST /doc_project`


#### Example Snippet

```rust
let client = sideko_rest_api::Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").unwrap())
    .with_cookie_auth(&std::env::var("API_KEY").unwrap());
let res = client
    .create_doc_project(sideko_rest_api::CreateDocProjectRequest {
        data: sideko_rest_api::models::NewDocProject {
            title: "my-company-docs".to_string(),
        },
    })
    .await;
```

    
### Deploy a new generated version of documentation with linked guides & APIs
Deploys a new generated version of documentation with linked guides & APIs

**API Endpoint**: `POST /doc_project/{project_id_or_name}/deployment`


#### Example Snippet

```rust
let client = sideko_rest_api::Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").unwrap())
    .with_cookie_auth(&std::env::var("API_KEY").unwrap());
let res = client
    .trigger_deployment(sideko_rest_api::TriggerDeploymentRequest {
        project_id_or_name: "string".to_string(),
        data: sideko_rest_api::models::NewDeployment {
            doc_version_id: Some("3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a".to_string()),
            target: sideko_rest_api::models::DeploymentTargetEnum::Preview,
        },
    })
    .await;
```

    
### Grant role to a user for a doc project.


**API Endpoint**: `POST /doc_project/{project_id_or_name}/role`


#### Example Snippet

```rust
let client = sideko_rest_api::Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").unwrap())
    .with_cookie_auth(&std::env::var("API_KEY").unwrap());
let res = client
    .grant_doc_project_role(sideko_rest_api::GrantDocProjectRoleRequest {
        project_id_or_name: "string".to_string(),
        data: sideko_rest_api::models::NewProjectRole {
            role: sideko_rest_api::models::ProjectRoleEnum::Admin,
            user_id: "3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a".to_string(),
        },
    })
    .await;
```

    
### Create a guide for a specific version of a documentation project


**API Endpoint**: `POST /doc_project/{project_id_or_name}/version/{version_id}/guide`


#### Example Snippet

```rust
let client = sideko_rest_api::Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").unwrap())
    .with_cookie_auth(&std::env::var("API_KEY").unwrap());
let res = client
    .create_guide(sideko_rest_api::CreateGuideRequest {
        project_id_or_name: "string".to_string(),
        version_id: "string".to_string(),
        data: sideko_rest_api::models::NewGuide {
            content: "string".to_string(),
            is_parent: true,
            nav_label: "string".to_string(),
            next_id: Some("3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a".to_string()),
            parent_id: Some("3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a".to_string()),
            prev_id: Some("3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a".to_string()),
            slug: "string".to_string(),
        },
    })
    .await;
```

    
### Reorder guides for a specific version of a documentation project


**API Endpoint**: `POST /doc_project/{project_id_or_name}/version/{version_id}/guide/reorder`


#### Example Snippet

```rust
let client = sideko_rest_api::Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").unwrap())
    .with_cookie_auth(&std::env::var("API_KEY").unwrap());
let res = client
    .reorder_guides(sideko_rest_api::ReorderGuidesRequest {
        project_id_or_name: "string".to_string(),
        version_id: "string".to_string(),
        data: vec![
            sideko_rest_api::models::ReorderGuide { id :
            "3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a".to_string(), order : 123,
            parent_id : Some("3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a".to_string()) }
        ],
    })
    .await;
```

    
### Create a new organization


**API Endpoint**: `POST /organization`


#### Example Snippet

```rust
let client = sideko_rest_api::Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").unwrap())
    .with_cookie_auth(&std::env::var("API_KEY").unwrap());
let res = client
    .create_organization(sideko_rest_api::CreateOrganizationRequest {
        data: sideko_rest_api::models::NewOrganization {
            name: "string".to_string(),
            subdomain: "string".to_string(),
        },
    })
    .await;
```

    
### 
Add a assets like logos or other media to an organization

**API Endpoint**: `POST /organization/asset`


#### Example Snippet

```rust
let client = sideko_rest_api::Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").unwrap())
    .with_cookie_auth(&std::env::var("API_KEY").unwrap());
let res = client
    .upload_assets(sideko_rest_api::UploadAssetsRequest {
        data: sideko_rest_api::models::File {
            file: sideko_rest_api::UploadFile::from_path("tests/file.pdf").unwrap(),
        },
    })
    .await;
```

    
### Create the initial version of a managed SDK


**API Endpoint**: `POST /sdk`


#### Example Snippet

```rust
let client = sideko_rest_api::Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").unwrap())
    .with_cookie_auth(&std::env::var("API_KEY").unwrap());
let res = client
    .create_sdk(sideko_rest_api::CreateSdkRequest {
        data: sideko_rest_api::models::SdkProject {
            api_project_version_id: "3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a"
                .to_string(),
            language: sideko_rest_api::models::GenerationLanguageEnum::Go,
            name: "my-api-python".to_string(),
            semver: "1.0.0".to_string(),
        },
    })
    .await;
```

    
### Update an SDK to match a new specification from an api-project


**API Endpoint**: `POST /sdk/{name}/{semver}`


#### Example Snippet

```rust
let client = sideko_rest_api::Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").unwrap())
    .with_cookie_auth(&std::env::var("API_KEY").unwrap());
let res = client
    .update_sdk(sideko_rest_api::UpdateSdkRequest {
        name: "my-python-sdk".to_string(),
        semver: "0.1.1".to_string(),
        data: sideko_rest_api::models::File {
            file: sideko_rest_api::UploadFile::from_path("tests/file.pdf").unwrap(),
        },
        ..Default::default()
    })
    .await;
```

    
### Generate SDK outside of Sideko versioning flow


**API Endpoint**: `POST /stateless/generate_sdk`


#### Example Snippet

```rust
let client = sideko_rest_api::Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").unwrap())
    .with_cookie_auth(&std::env::var("API_KEY").unwrap());
let res = client
    .stateless_generate_sdk(sideko_rest_api::StatelessGenerateSdkRequest {
        data: sideko_rest_api::models::StatelessGenerateSdk {
            base_url: Some("http://127.0.0.1:8080/api".to_string()),
            language: sideko_rest_api::models::GenerationLanguageEnum::Go,
            openapi: "openapi: 3.0.0".to_string(),
            package_name: Some("my_sdk".to_string()),
            tests_mock_server_url: Some("http://127.0.0.1:8080/mock".to_string()),
        },
    })
    .await;
```

    
### Invite a user to an organization with a specific role


**API Endpoint**: `POST /user/invite`


#### Example Snippet

```rust
let client = sideko_rest_api::Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").unwrap())
    .with_cookie_auth(&std::env::var("API_KEY").unwrap());
let res = client
    .invite_user(sideko_rest_api::InviteUserRequest {
        data: sideko_rest_api::models::Invite {
            email: "user@example.com".to_string(),
            role: sideko_rest_api::models::OrganizationRoleEnum::Admin,
        },
    })
    .await;
```

    
### Create a new service account with a set of project permissions


**API Endpoint**: `POST /user/service_account`


#### Example Snippet

```rust
let client = sideko_rest_api::Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").unwrap())
    .with_cookie_auth(&std::env::var("API_KEY").unwrap());
let res = client
    .create_service_account(sideko_rest_api::CreateServiceAccountRequest {
        data: sideko_rest_api::models::CreateServiceAccount {
            name: "Documentation Publisher Service Account".to_string(),
            project_roles: vec![
                sideko_rest_api::models::UserProjectRole { project_id_or_name :
                "string".to_string(), project_type :
                sideko_rest_api::models::ProjectTypeEnum::Api, role :
                sideko_rest_api::models::ProjectRoleEnum::Admin }
            ],
        },
    })
    .await;
```

    
### webhooks sent to sideko by vercel


**API Endpoint**: `POST /webhook/vercel`


#### Example Snippet

```rust
let client = sideko_rest_api::Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").unwrap())
    .with_cookie_auth(&std::env::var("API_KEY").unwrap());
let res = client
    .vercel_webhook(sideko_rest_api::VercelWebhookRequest {
        data: serde_json::json!({}),
    })
    .await;
```

    
### Update a document project theme


**API Endpoint**: `PUT /doc_project/{project_id_or_name}/theme`


#### Example Snippet

```rust
let client = sideko_rest_api::Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").unwrap())
    .with_cookie_auth(&std::env::var("API_KEY").unwrap());
let res = client
    .update_doc_project_theme(sideko_rest_api::UpdateDocProjectThemeRequest {
        project_id_or_name: "string".to_string(),
        data: sideko_rest_api::models::ThemeValues {
            api_reference_group_variant: Some("grouped".to_string()),
            dark_active_button_bg_color: Some("#FFFFFF".to_string()),
            dark_active_button_text_color: Some("#FFFFFF".to_string()),
            dark_bg_color: Some("#FFFFFF".to_string()),
            dark_navbar_color: Some("#FFFFFF".to_string()),
            dark_navbar_text_color: Some("#FFFFFF".to_string()),
            light_active_button_bg_color: Some("#FFFFFF".to_string()),
            light_active_button_text_color: Some("#FFFFFF".to_string()),
            light_bg_color: Some("#FFFFFF".to_string()),
            light_navbar_color: Some("#FFFFFF".to_string()),
            light_navbar_text_color: Some("#FFFFFF".to_string()),
        },
    })
    .await;
```

    
### 
Update  documentation project theme configured at the organization level

**API Endpoint**: `PUT /organization/theme`


#### Example Snippet

```rust
let client = sideko_rest_api::Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").unwrap())
    .with_cookie_auth(&std::env::var("API_KEY").unwrap());
let res = client
    .update_organization_theme(sideko_rest_api::UpdateOrganizationThemeRequest {
        data: sideko_rest_api::models::ThemeValues {
            api_reference_group_variant: Some("grouped".to_string()),
            dark_active_button_bg_color: Some("#FFFFFF".to_string()),
            dark_active_button_text_color: Some("#FFFFFF".to_string()),
            dark_bg_color: Some("#FFFFFF".to_string()),
            dark_navbar_color: Some("#FFFFFF".to_string()),
            dark_navbar_text_color: Some("#FFFFFF".to_string()),
            light_active_button_bg_color: Some("#FFFFFF".to_string()),
            light_active_button_text_color: Some("#FFFFFF".to_string()),
            light_bg_color: Some("#FFFFFF".to_string()),
            light_navbar_color: Some("#FFFFFF".to_string()),
            light_navbar_text_color: Some("#FFFFFF".to_string()),
        },
    })
    .await;
```

    