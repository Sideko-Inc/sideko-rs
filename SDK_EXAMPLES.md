
# SDK Usage Examples


## Module 
            
### Removes an API link


**API Endpoint**: `DELETE /api_link/{id}`


#### Example Snippet

```rust
let client = sideko_rest_api::Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").unwrap())
    .with_cookie_auth(&std::env::var("API_KEY").unwrap());
let res = client
    .delete_api_link(sideko_rest_api::DeleteApiLinkRequest {
        id: "3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a".to_string(),
    })
    .await;
```

    
### Deletes the api group and all its links


**API Endpoint**: `DELETE /api_link_group/{id}`


#### Example Snippet

```rust
let client = sideko_rest_api::Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").unwrap())
    .with_cookie_auth(&std::env::var("API_KEY").unwrap());
let res = client
    .delete_api_link_group(sideko_rest_api::DeleteApiLinkGroupRequest {
        id: "3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a".to_string(),
    })
    .await;
```

    
### Delete a specific Documentation Project


**API Endpoint**: `DELETE /doc_project/{doc_name}`


#### Example Snippet

```rust
let client = sideko_rest_api::Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").unwrap())
    .with_cookie_auth(&std::env::var("API_KEY").unwrap());
let res = client
    .delete_doc_project(sideko_rest_api::DeleteDocProjectRequest {
        doc_name: "my-project".to_string(),
    })
    .await;
```

    
### Delete a specific guide for a specific version of a documentation project


**API Endpoint**: `DELETE /doc_project/{doc_name}/version/{doc_version}/guide/{guide_id}`


#### Example Snippet

```rust
let client = sideko_rest_api::Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").unwrap())
    .with_cookie_auth(&std::env::var("API_KEY").unwrap());
let res = client
    .delete_guide(sideko_rest_api::DeleteGuideRequest {
        doc_name: "my-project".to_string(),
        doc_version: sideko_rest_api::models::IdOrInt::Str(
            "3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a".to_string(),
        ),
        guide_id: "3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a".to_string(),
    })
    .await;
```

    
### Delete href for a specific guide for a specific version of a documentation project


**API Endpoint**: `DELETE /doc_project/{doc_name}/version/{doc_version}/guide/{guide_id}/href`


#### Example Snippet

```rust
let client = sideko_rest_api::Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").unwrap())
    .with_cookie_auth(&std::env::var("API_KEY").unwrap());
let res = client
    .delete_guide_href(sideko_rest_api::DeleteGuideHrefRequest {
        doc_name: "my-project".to_string(),
        doc_version: sideko_rest_api::models::IdOrInt::Str(
            "3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a".to_string(),
        ),
        guide_id: "3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a".to_string(),
        variant: sideko_rest_api::models::GuideHrefVariantEnum::Next,
    })
    .await;
```

    
### Delete Asset
Delete a media asset in your organization

**API Endpoint**: `DELETE /organization/asset/{id}`


#### Example Snippet

```rust
let client = sideko_rest_api::Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").unwrap())
    .with_cookie_auth(&std::env::var("API_KEY").unwrap());
let res = client
    .delete_asset(sideko_rest_api::DeleteAssetRequest {
        id: "3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a".to_string(),
    })
    .await;
```

    
### Delete a service account


**API Endpoint**: `DELETE /user/service_account/{id}`


#### Example Snippet

```rust
let client = sideko_rest_api::Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").unwrap())
    .with_cookie_auth(&std::env::var("API_KEY").unwrap());
let res = client
    .delete_service_account(sideko_rest_api::DeleteServiceAccountRequest {
        id: "3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a".to_string(),
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

    
### List API links for doc version


**API Endpoint**: `GET /api_link`


#### Example Snippet

```rust
let client = sideko_rest_api::Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").unwrap())
    .with_cookie_auth(&std::env::var("API_KEY").unwrap());
let res = client
    .list_api_links(sideko_rest_api::ListApiLinksRequest {
        doc_version: sideko_rest_api::models::IdOrInt::Str(
            "3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a".to_string(),
        ),
    })
    .await;
```

    
### Retrieve single API link


**API Endpoint**: `GET /api_link/{id}`


#### Example Snippet

```rust
let client = sideko_rest_api::Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").unwrap())
    .with_cookie_auth(&std::env::var("API_KEY").unwrap());
let res = client
    .get_api_link(sideko_rest_api::GetApiLinkRequest {
        id: "3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a".to_string(),
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
        doc_version: sideko_rest_api::models::IdOrInt::Str(
            "3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a".to_string(),
        ),
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


**API Endpoint**: `GET /doc_project/{doc_name}`


#### Example Snippet

```rust
let client = sideko_rest_api::Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").unwrap())
    .with_cookie_auth(&std::env::var("API_KEY").unwrap());
let res = client
    .get_doc_project(sideko_rest_api::GetDocProjectRequest {
        doc_name: "my-project".to_string(),
    })
    .await;
```

    
### List deployments for a specific documentation project
Retrieves all deployments for a doc project

**API Endpoint**: `GET /doc_project/{doc_name}/deployment`


#### Example Snippet

```rust
let client = sideko_rest_api::Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").unwrap())
    .with_cookie_auth(&std::env::var("API_KEY").unwrap());
let res = client
    .list_deployments(sideko_rest_api::ListDeploymentsRequest {
        doc_name: "my-project".to_string(),
        ..Default::default()
    })
    .await;
```

    
### Get a specific deployment for a specific documentation project
Retrieves single deployment

**API Endpoint**: `GET /doc_project/{doc_name}/deployment/{deployment_id}`


#### Example Snippet

```rust
let client = sideko_rest_api::Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").unwrap())
    .with_cookie_auth(&std::env::var("API_KEY").unwrap());
let res = client
    .get_deployment(sideko_rest_api::GetDeploymentRequest {
        doc_name: "my-project".to_string(),
        deployment_id: "3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a".to_string(),
    })
    .await;
```

    
### A simple check to see if the requesting user has access to the preview doc project


**API Endpoint**: `GET /doc_project/{doc_name}/preview`


#### Example Snippet

```rust
let client = sideko_rest_api::Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").unwrap())
    .with_cookie_auth(&std::env::var("API_KEY").unwrap());
let res = client
    .check_preview(sideko_rest_api::CheckPreviewRequest {
        doc_name: "my-project".to_string(),
    })
    .await;
```

    
### Get the theme attached to a documentation project


**API Endpoint**: `GET /doc_project/{doc_name}/theme`


#### Example Snippet

```rust
let client = sideko_rest_api::Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").unwrap())
    .with_cookie_auth(&std::env::var("API_KEY").unwrap());
let res = client
    .get_doc_project_theme(sideko_rest_api::GetDocProjectThemeRequest {
        doc_name: "my-project".to_string(),
    })
    .await;
```

    
### List versions of a specific Documentation Project


**API Endpoint**: `GET /doc_project/{doc_name}/version`


#### Example Snippet

```rust
let client = sideko_rest_api::Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").unwrap())
    .with_cookie_auth(&std::env::var("API_KEY").unwrap());
let res = client
    .list_doc_versions(sideko_rest_api::ListDocVersionsRequest {
        doc_name: "my-project".to_string(),
    })
    .await;
```

    
### Get a specific version of an Documentation Project


**API Endpoint**: `GET /doc_project/{doc_name}/version/{doc_version}`


#### Example Snippet

```rust
let client = sideko_rest_api::Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").unwrap())
    .with_cookie_auth(&std::env::var("API_KEY").unwrap());
let res = client
    .get_doc_version(sideko_rest_api::GetDocVersionRequest {
        doc_name: "my-project".to_string(),
        doc_version: sideko_rest_api::models::IdOrInt::Str(
            "3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a".to_string(),
        ),
    })
    .await;
```

    
### List guides for a specific version of a documentation project


**API Endpoint**: `GET /doc_project/{doc_name}/version/{doc_version}/guide`


#### Example Snippet

```rust
let client = sideko_rest_api::Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").unwrap())
    .with_cookie_auth(&std::env::var("API_KEY").unwrap());
let res = client
    .list_guides(sideko_rest_api::ListGuidesRequest {
        doc_name: "my-project".to_string(),
        doc_version: sideko_rest_api::models::IdOrInt::Str(
            "3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a".to_string(),
        ),
    })
    .await;
```

    
### Get a specific guide for a specific version of a documentation project


**API Endpoint**: `GET /doc_project/{doc_name}/version/{doc_version}/guide/{guide_id}`


#### Example Snippet

```rust
let client = sideko_rest_api::Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").unwrap())
    .with_cookie_auth(&std::env::var("API_KEY").unwrap());
let res = client
    .get_guide(sideko_rest_api::GetGuideRequest {
        doc_name: "my-project".to_string(),
        doc_version: sideko_rest_api::models::IdOrInt::Str(
            "3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a".to_string(),
        ),
        guide_id: "3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a".to_string(),
    })
    .await;
```

    
### Get content for a specific guide for a specific version of a documentation project


**API Endpoint**: `GET /doc_project/{doc_name}/version/{doc_version}/guide/{guide_id}/content`


#### Example Snippet

```rust
let client = sideko_rest_api::Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").unwrap())
    .with_cookie_auth(&std::env::var("API_KEY").unwrap());
let res = client
    .get_guide_content(sideko_rest_api::GetGuideContentRequest {
        doc_name: "my-project".to_string(),
        doc_version: sideko_rest_api::models::IdOrInt::Str(
            "3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a".to_string(),
        ),
        guide_id: "3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a".to_string(),
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

    
### List Assets
Get all media assets for an organization

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
Get documentation project theme configured at the organization level

**API Endpoint**: `GET /organization/theme`


#### Example Snippet

```rust
let client = sideko_rest_api::Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").unwrap())
    .with_cookie_auth(&std::env::var("API_KEY").unwrap());
let res = client.get_organization_theme().await;
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

    
### Get all of your Service Accounts


**API Endpoint**: `GET /user/service_account`


#### Example Snippet

```rust
let client = sideko_rest_api::Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").unwrap())
    .with_cookie_auth(&std::env::var("API_KEY").unwrap());
let res = client.list_service_accounts().await;
```

    
### Get all of your Service Accounts


**API Endpoint**: `GET /user/service_account/{id}`


#### Example Snippet

```rust
let client = sideko_rest_api::Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").unwrap())
    .with_cookie_auth(&std::env::var("API_KEY").unwrap());
let res = client
    .get_service_account(sideko_rest_api::GetServiceAccountRequest {
        id: "3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a".to_string(),
    })
    .await;
```

    
### Updates an API link


**API Endpoint**: `PATCH /api_link/{id}`


#### Example Snippet

```rust
let client = sideko_rest_api::Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").unwrap())
    .with_cookie_auth(&std::env::var("API_KEY").unwrap());
let res = client
    .update_api_link(sideko_rest_api::UpdateApiLinkRequest {
        id: "3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a".to_string(),
        data: sideko_rest_api::models::UpdateApiLink {
            api_version: Some(sideko_rest_api::models::UpdateApiLinkApiVersion {
                api_id: "my-api".to_string(),
                version: "0.1.0".to_string(),
            }),
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


**API Endpoint**: `PATCH /api_link_group/{id}`


#### Example Snippet

```rust
let client = sideko_rest_api::Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").unwrap())
    .with_cookie_auth(&std::env::var("API_KEY").unwrap());
let res = client
    .update_api_link_group(sideko_rest_api::UpdateApiLinkGroupRequest {
        id: "3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a".to_string(),
        data: sideko_rest_api::models::UpdateApiLinkGroup {
            nav_label: Some("string".to_string()),
            slug: Some("string".to_string()),
        },
    })
    .await;
```

    
### Update an existing Documentation Project


**API Endpoint**: `PATCH /doc_project/{doc_name}`


#### Example Snippet

```rust
let client = sideko_rest_api::Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").unwrap())
    .with_cookie_auth(&std::env::var("API_KEY").unwrap());
let res = client
    .update_doc_project(sideko_rest_api::UpdateDocProjectRequest {
        doc_name: "my-project".to_string(),
        data: sideko_rest_api::models::UpdateDocProject {
            logos: Some(sideko_rest_api::models::UpdateDocProjectLogos {
                dark: Some("3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a".to_string()),
                favicon: Some("3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a".to_string()),
                light: Some("3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a".to_string()),
            }),
            name: Some("my-company-docs".to_string()),
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
        },
    })
    .await;
```

    
### Update a specific guide for a specific version of a documentation project


**API Endpoint**: `PATCH /doc_project/{doc_name}/version/{doc_version}/guide/{guide_id}`


#### Example Snippet

```rust
let client = sideko_rest_api::Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").unwrap())
    .with_cookie_auth(&std::env::var("API_KEY").unwrap());
let res = client
    .update_guide(sideko_rest_api::UpdateGuideRequest {
        doc_name: "my-project".to_string(),
        doc_version: sideko_rest_api::models::IdOrInt::Str(
            "3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a".to_string(),
        ),
        guide_id: "3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a".to_string(),
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

    
### Update Asset
Update a media asset in your organization

**API Endpoint**: `PATCH /organization/asset/{id}`


#### Example Snippet

```rust
let client = sideko_rest_api::Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").unwrap())
    .with_cookie_auth(&std::env::var("API_KEY").unwrap());
let res = client
    .update_asset(sideko_rest_api::UpdateAssetRequest {
        id: "3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a".to_string(),
        data: sideko_rest_api::models::UpdateAsset {
            name: Some("string".to_string()),
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
            name: "my-company-docs".to_string(),
        },
    })
    .await;
```

    
### Deploy a new generated version of documentation with linked guides & APIs
Deploys a new generated version of documentation with linked guides & APIs

**API Endpoint**: `POST /doc_project/{doc_name}/deployment`


#### Example Snippet

```rust
let client = sideko_rest_api::Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").unwrap())
    .with_cookie_auth(&std::env::var("API_KEY").unwrap());
let res = client
    .trigger_deployment(sideko_rest_api::TriggerDeploymentRequest {
        doc_name: "my-project".to_string(),
        data: sideko_rest_api::models::NewDeployment {
            doc_version_id: Some("3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a".to_string()),
            target: sideko_rest_api::models::DeploymentTargetEnum::Preview,
        },
    })
    .await;
```

    
### Create a guide for a specific version of a documentation project


**API Endpoint**: `POST /doc_project/{doc_name}/version/{doc_version}/guide`


#### Example Snippet

```rust
let client = sideko_rest_api::Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").unwrap())
    .with_cookie_auth(&std::env::var("API_KEY").unwrap());
let res = client
    .create_guide(sideko_rest_api::CreateGuideRequest {
        doc_name: "my-project".to_string(),
        doc_version: sideko_rest_api::models::IdOrInt::Str(
            "3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a".to_string(),
        ),
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


**API Endpoint**: `POST /doc_project/{doc_name}/version/{doc_version}/guide/reorder`


#### Example Snippet

```rust
let client = sideko_rest_api::Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").unwrap())
    .with_cookie_auth(&std::env::var("API_KEY").unwrap());
let res = client
    .reorder_guides(sideko_rest_api::ReorderGuidesRequest {
        doc_name: "my-project".to_string(),
        doc_version: sideko_rest_api::models::IdOrInt::Str(
            "3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a".to_string(),
        ),
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

    
### Upload Assets
Add a media asset like logos or other media to an organization

**API Endpoint**: `POST /organization/asset`


#### Example Snippet

```rust
let client = sideko_rest_api::Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").unwrap())
    .with_cookie_auth(&std::env::var("API_KEY").unwrap());
let res = client
    .upload_assets(sideko_rest_api::UploadAssetsRequest {
        data: sideko_rest_api::models::File {
            file: sideko_rest_api::UploadFile::from_path("uploads/file.pdf").unwrap(),
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
            role_definition_id: sideko_rest_api::models::RoleDefinitionIdEnum::ApiProjectAdmin,
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
        data: sideko_rest_api::models::NewServiceAccount {
            name: "Documentation Publisher Service Account".to_string(),
            object_roles: vec![
                sideko_rest_api::models::ObjectRole { object_id :
                "3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a".to_string(), object_type :
                sideko_rest_api::models::ObjectTypeEnum::ApiProject,
                role_definition_id :
                sideko_rest_api::models::RoleDefinitionIdEnum::ApiProjectAdmin }
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


**API Endpoint**: `PUT /doc_project/{doc_name}/theme`


#### Example Snippet

```rust
let client = sideko_rest_api::Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").unwrap())
    .with_cookie_auth(&std::env::var("API_KEY").unwrap());
let res = client
    .update_doc_project_theme(sideko_rest_api::UpdateDocProjectThemeRequest {
        doc_name: "my-project".to_string(),
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

    
## Module api
            
### Delete a API specification collection


**API Endpoint**: `DELETE /api/{api_name}`


#### Example Snippet

```rust
let client = sideko_rest_api::Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").unwrap())
    .with_cookie_auth(&std::env::var("API_KEY").unwrap());
let res = client
    .api()
    .delete(sideko_rest_api::resources::api::DeleteRequest {
        api_name: "my-project".to_string(),
    })
    .await;
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


**API Endpoint**: `GET /api/{api_name}`


#### Example Snippet

```rust
let client = sideko_rest_api::Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").unwrap())
    .with_cookie_auth(&std::env::var("API_KEY").unwrap());
let res = client
    .api()
    .get(sideko_rest_api::resources::api::GetRequest {
        api_name: "my-project".to_string(),
    })
    .await;
```

    
### Update an existing API Specification collection


**API Endpoint**: `PATCH /api/{api_name}`


#### Example Snippet

```rust
let client = sideko_rest_api::Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").unwrap())
    .with_cookie_auth(&std::env::var("API_KEY").unwrap());
let res = client
    .api()
    .patch(sideko_rest_api::resources::api::PatchRequest {
        api_name: "my-project".to_string(),
        data: sideko_rest_api::models::UpdateApi {
            name: Some("my-new-api-name".to_string()),
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
            name: "my-api-spec".to_string(),
        },
    })
    .await;
```

    
## Module api.spec
            
### Delete an API Specification and it's associated metadata


**API Endpoint**: `DELETE /api/{api_name}/spec/{api_version}`


#### Example Snippet

```rust
let client = sideko_rest_api::Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").unwrap())
    .with_cookie_auth(&std::env::var("API_KEY").unwrap());
let res = client
    .api()
    .spec()
    .delete(sideko_rest_api::resources::api::spec::DeleteRequest {
        api_name: "my-project".to_string(),
        api_version: sideko_rest_api::models::ApiVersion::StrEnum(
            sideko_rest_api::models::VersionTypeEnum::Latest,
        ),
    })
    .await;
```

    
### List specs of a collection


**API Endpoint**: `GET /api/{api_name}/spec`


#### Example Snippet

```rust
let client = sideko_rest_api::Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").unwrap())
    .with_cookie_auth(&std::env::var("API_KEY").unwrap());
let res = client
    .api()
    .spec()
    .list(sideko_rest_api::resources::api::spec::ListRequest {
        api_name: "my-project".to_string(),
    })
    .await;
```

    
### Get API specification metadata


**API Endpoint**: `GET /api/{api_name}/spec/{api_version}`


#### Example Snippet

```rust
let client = sideko_rest_api::Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").unwrap())
    .with_cookie_auth(&std::env::var("API_KEY").unwrap());
let res = client
    .api()
    .spec()
    .get(sideko_rest_api::resources::api::spec::GetRequest {
        api_name: "my-project".to_string(),
        api_version: sideko_rest_api::models::ApiVersion::StrEnum(
            sideko_rest_api::models::VersionTypeEnum::Latest,
        ),
    })
    .await;
```

    
### Update an API Specification and/or metadata


**API Endpoint**: `PATCH /api/{api_name}/spec/{api_version}`


#### Example Snippet

```rust
let client = sideko_rest_api::Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").unwrap())
    .with_cookie_auth(&std::env::var("API_KEY").unwrap());
let res = client
    .api()
    .spec()
    .patch(sideko_rest_api::resources::api::spec::PatchRequest {
        api_name: "my-project".to_string(),
        api_version: sideko_rest_api::models::ApiVersion::StrEnum(
            sideko_rest_api::models::VersionTypeEnum::Latest,
        ),
        data: sideko_rest_api::models::UpdateApiSpec {
            mock_server_enabled: Some(true),
            notes: Some(
                "<p>This version includes a number of excellent improvements</p>"
                    .to_string(),
            ),
            openapi: Some(
                sideko_rest_api::UploadFile::from_path("openapi.yaml").unwrap(),
            ),
            version: Some("string".to_string()),
        },
    })
    .await;
```

    
### Add a new API specification


**API Endpoint**: `POST /api/{api_name}/spec`


#### Example Snippet

```rust
let client = sideko_rest_api::Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").unwrap())
    .with_cookie_auth(&std::env::var("API_KEY").unwrap());
let res = client
    .api()
    .spec()
    .create(sideko_rest_api::resources::api::spec::CreateRequest {
        api_name: "my-project".to_string(),
        data: sideko_rest_api::models::NewApiSpec {
            mock_server_enabled: Some(true),
            notes: Some(
                "<p>This version includes a number of excellent improvements</p>"
                    .to_string(),
            ),
            openapi: sideko_rest_api::UploadFile::from_path("openapi.yaml").unwrap(),
            version: sideko_rest_api::models::VersionOrBump::StrEnum(
                sideko_rest_api::models::VersionBumpEnum::Major,
            ),
        },
    })
    .await;
```

    
## Module role
            
### Delete role and associated permissions


**API Endpoint**: `DELETE /role/{id}`


#### Example Snippet

```rust
let client = sideko_rest_api::Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").unwrap())
    .with_cookie_auth(&std::env::var("API_KEY").unwrap());
let res = client
    .role()
    .delete(sideko_rest_api::resources::role::DeleteRequest {
        id: "3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a".to_string(),
    })
    .await;
```

    
### List roles


**API Endpoint**: `GET /role`


#### Example Snippet

```rust
let client = sideko_rest_api::Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").unwrap())
    .with_cookie_auth(&std::env::var("API_KEY").unwrap());
let res = client
    .role()
    .list(sideko_rest_api::resources::role::ListRequest {
        ..Default::default()
    })
    .await;
```

    
### Create new role


**API Endpoint**: `POST /role`


#### Example Snippet

```rust
let client = sideko_rest_api::Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").unwrap())
    .with_cookie_auth(&std::env::var("API_KEY").unwrap());
let res = client
    .role()
    .create(sideko_rest_api::resources::role::CreateRequest {
        data: sideko_rest_api::models::NewRole {
            object_id: "3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a".to_string(),
            object_type: sideko_rest_api::models::ObjectTypeEnum::ApiProject,
            role_definition_id: sideko_rest_api::models::RoleDefinitionIdEnum::ApiProjectAdmin,
            user_id: "3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a".to_string(),
        },
    })
    .await;
```

    
## Module api.spec.openapi
            
### Get OpenAPI specification


**API Endpoint**: `GET /api/{api_name}/spec/{api_version}/openapi`


#### Example Snippet

```rust
let client = sideko_rest_api::Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").unwrap())
    .with_cookie_auth(&std::env::var("API_KEY").unwrap());
let res = client
    .api()
    .spec()
    .openapi()
    .get_openapi(sideko_rest_api::resources::api::spec::openapi::GetOpenapiRequest {
        api_name: "my-project".to_string(),
        api_version: sideko_rest_api::models::ApiVersion::StrEnum(
            sideko_rest_api::models::VersionTypeEnum::Latest,
        ),
    })
    .await;
```

    
## Module api.spec.stats
            
### Get Stats about the specification


**API Endpoint**: `GET /api/{api_name}/spec/{api_version}/stats`


#### Example Snippet

```rust
let client = sideko_rest_api::Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").unwrap())
    .with_cookie_auth(&std::env::var("API_KEY").unwrap());
let res = client
    .api()
    .spec()
    .stats()
    .get_stats(sideko_rest_api::resources::api::spec::stats::GetStatsRequest {
        api_name: "my-project".to_string(),
        api_version: sideko_rest_api::models::ApiVersion::StrEnum(
            sideko_rest_api::models::VersionTypeEnum::Latest,
        ),
    })
    .await;
```

    
## Module sdk
            
### 


**API Endpoint**: `GET /sdk`


#### Example Snippet

```rust
let client = sideko_rest_api::Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").unwrap())
    .with_cookie_auth(&std::env::var("API_KEY").unwrap());
let res = client
    .sdk()
    .list(sideko_rest_api::resources::sdk::ListRequest {
        ..Default::default()
    })
    .await;
```

    
### 


**API Endpoint**: `POST /sdk`


#### Example Snippet

```rust
let client = sideko_rest_api::Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").unwrap())
    .with_cookie_auth(&std::env::var("API_KEY").unwrap());
let res = client
    .sdk()
    .generate(sideko_rest_api::resources::sdk::GenerateRequest {
        data: sideko_rest_api::models::NewSdk {
            api_version: Some(
                sideko_rest_api::models::ApiVersion::StrEnum(
                    sideko_rest_api::models::VersionTypeEnum::Latest,
                ),
            ),
            config: sideko_rest_api::UploadFile::from_path("uploads/file.pdf")
                .unwrap(),
            language: sideko_rest_api::models::SdkLanguageEnum::Go,
            sdk_version: Some("0.1.0".to_string()),
        },
    })
    .await;
```

    
### 


**API Endpoint**: `POST /sdk/update`


#### Example Snippet

```rust
let client = sideko_rest_api::Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").unwrap())
    .with_cookie_auth(&std::env::var("API_KEY").unwrap());
let res = client
    .sdk()
    .update(sideko_rest_api::resources::sdk::UpdateRequest {
        data: sideko_rest_api::models::UpdateSdk {
            api_version: Some(
                sideko_rest_api::models::ApiVersion::StrEnum(
                    sideko_rest_api::models::VersionTypeEnum::Latest,
                ),
            ),
            config: sideko_rest_api::UploadFile::from_path("uploads/file.pdf")
                .unwrap(),
            prev_sdk_git: sideko_rest_api::UploadFile::from_path("uploads/file.pdf")
                .unwrap(),
            prev_sdk_id: "3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a".to_string(),
            sdk_version: sideko_rest_api::models::VersionOrBump::StrEnum(
                sideko_rest_api::models::VersionBumpEnum::Major,
            ),
        },
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
    .sdk()
    .generate_stateless(sideko_rest_api::resources::sdk::GenerateStatelessRequest {
        data: sideko_rest_api::models::NewStatelessSdk {
            base_url: Some("http://127.0.0.1:8080/api".to_string()),
            language: sideko_rest_api::models::SdkLanguageEnum::Go,
            openapi: sideko_rest_api::UploadFile::from_path("openapi.yaml").unwrap(),
            package_name: Some("my_sdk".to_string()),
        },
    })
    .await;
```

    
## Module sdk.config
            
### 
Creates a sdk config with default configurations for the api/api version

**API Endpoint**: `POST /sdk/config/init`


#### Example Snippet

```rust
let client = sideko_rest_api::Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").unwrap())
    .with_cookie_auth(&std::env::var("API_KEY").unwrap());
let res = client
    .sdk()
    .config()
    .init(sideko_rest_api::resources::sdk::config::InitRequest {
        data: sideko_rest_api::models::InitSdkConfig {
            api_name: "my-project".to_string(),
            api_version: Some(
                sideko_rest_api::models::ApiVersion::StrEnum(
                    sideko_rest_api::models::VersionTypeEnum::Latest,
                ),
            ),
        },
    })
    .await;
```

    
### 
Updates provided config with missing default configurations for the api version

**API Endpoint**: `POST /sdk/config/sync`


#### Example Snippet

```rust
let client = sideko_rest_api::Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").unwrap())
    .with_cookie_auth(&std::env::var("API_KEY").unwrap());
let res = client
    .sdk()
    .config()
    .sync(sideko_rest_api::resources::sdk::config::SyncRequest {
        data: sideko_rest_api::models::SyncSdkConfig {
            api_version: Some(
                sideko_rest_api::models::ApiVersion::StrEnum(
                    sideko_rest_api::models::VersionTypeEnum::Latest,
                ),
            ),
            config: sideko_rest_api::UploadFile::from_path("uploads/file.pdf")
                .unwrap(),
        },
    })
    .await;
```

    