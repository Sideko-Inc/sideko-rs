#[serial_test::serial]
#[tokio::test]
async fn test_delete_api_link_204_generated_success() {
    let client = sideko_rest_api::Client::default()
        .with_api_key_auth(&std::env::var("API_KEY").unwrap())
        .with_cookie_auth(&std::env::var("API_KEY").unwrap());
    let res = client
        .delete_api_link(sideko_rest_api::DeleteApiLinkRequest {
            id: "3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a".to_string(),
        })
        .await;
    println!("{:?}", res);
    assert!(res.is_ok());
}
#[serial_test::serial]
#[tokio::test]
async fn test_delete_api_link_group_204_generated_success() {
    let client = sideko_rest_api::Client::default()
        .with_api_key_auth(&std::env::var("API_KEY").unwrap())
        .with_cookie_auth(&std::env::var("API_KEY").unwrap());
    let res = client
        .delete_api_link_group(sideko_rest_api::DeleteApiLinkGroupRequest {
            id: "3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a".to_string(),
        })
        .await;
    println!("{:?}", res);
    assert!(res.is_ok());
}
#[serial_test::serial]
#[tokio::test]
async fn test_delete_doc_project_204_generated_success() {
    let client = sideko_rest_api::Client::default()
        .with_api_key_auth(&std::env::var("API_KEY").unwrap())
        .with_cookie_auth(&std::env::var("API_KEY").unwrap());
    let res = client
        .delete_doc_project(sideko_rest_api::DeleteDocProjectRequest {
            doc_name: "my-project".to_string(),
        })
        .await;
    println!("{:?}", res);
    assert!(res.is_ok());
}
#[serial_test::serial]
#[tokio::test]
async fn test_delete_guide_204_generated_success() {
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
    println!("{:?}", res);
    assert!(res.is_ok());
}
#[serial_test::serial]
#[tokio::test]
async fn test_delete_guide_href_204_generated_success() {
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
    println!("{:?}", res);
    assert!(res.is_ok());
}
#[serial_test::serial]
#[tokio::test]
async fn test_delete_asset_204_generated_success() {
    let client = sideko_rest_api::Client::default()
        .with_api_key_auth(&std::env::var("API_KEY").unwrap())
        .with_cookie_auth(&std::env::var("API_KEY").unwrap());
    let res = client
        .delete_asset(sideko_rest_api::DeleteAssetRequest {
            id: "3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a".to_string(),
        })
        .await;
    println!("{:?}", res);
    assert!(res.is_ok());
}
#[serial_test::serial]
#[tokio::test]
async fn test_delete_service_account_204_generated_success() {
    let client = sideko_rest_api::Client::default()
        .with_api_key_auth(&std::env::var("API_KEY").unwrap())
        .with_cookie_auth(&std::env::var("API_KEY").unwrap());
    let res = client
        .delete_service_account(sideko_rest_api::DeleteServiceAccountRequest {
            id: "3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a".to_string(),
        })
        .await;
    println!("{:?}", res);
    assert!(res.is_ok());
}
#[serial_test::serial]
#[tokio::test]
async fn test_health_check_200_generated_success() {
    let client = sideko_rest_api::Client::default()
        .with_api_key_auth(&std::env::var("API_KEY").unwrap())
        .with_cookie_auth(&std::env::var("API_KEY").unwrap());
    let res = client.health_check().await;
    println!("{:?}", res);
    assert!(res.is_ok());
}
#[serial_test::serial]
#[tokio::test]
async fn test_ping_check_200_generated_success() {
    let client = sideko_rest_api::Client::default()
        .with_api_key_auth(&std::env::var("API_KEY").unwrap())
        .with_cookie_auth(&std::env::var("API_KEY").unwrap());
    let res = client.ping_check().await;
    println!("{:?}", res);
    assert!(res.is_ok());
}
#[serial_test::serial]
#[tokio::test]
async fn test_list_api_links_200_generated_success() {
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
    println!("{:?}", res);
    assert!(res.is_ok());
}
#[serial_test::serial]
#[tokio::test]
async fn test_get_api_link_200_generated_success() {
    let client = sideko_rest_api::Client::default()
        .with_api_key_auth(&std::env::var("API_KEY").unwrap())
        .with_cookie_auth(&std::env::var("API_KEY").unwrap());
    let res = client
        .get_api_link(sideko_rest_api::GetApiLinkRequest {
            id: "3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a".to_string(),
        })
        .await;
    println!("{:?}", res);
    assert!(res.is_ok());
}
#[serial_test::serial]
#[tokio::test]
async fn test_list_api_link_groups_200_generated_success() {
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
    println!("{:?}", res);
    assert!(res.is_ok());
}
#[serial_test::serial]
#[tokio::test]
async fn test_exchange_code_for_key_200_generated_success() {
    let client = sideko_rest_api::Client::default()
        .with_api_key_auth(&std::env::var("API_KEY").unwrap())
        .with_cookie_auth(&std::env::var("API_KEY").unwrap());
    let res = client
        .exchange_code_for_key(sideko_rest_api::ExchangeCodeForKeyRequest {
            code: "string".to_string(),
        })
        .await;
    println!("{:?}", res);
    assert!(res.is_ok());
}
#[serial_test::serial]
#[tokio::test]
async fn test_login_callback_303_generated_success() {
    let client = sideko_rest_api::Client::default()
        .with_api_key_auth(&std::env::var("API_KEY").unwrap())
        .with_cookie_auth(&std::env::var("API_KEY").unwrap());
    let res = client
        .login_callback(sideko_rest_api::LoginCallbackRequest {
            code: "string".to_string(),
            ..Default::default()
        })
        .await;
    println!("{:?}", res);
    assert!(res.is_ok());
}
#[serial_test::serial]
#[tokio::test]
async fn test_login_url_303_generated_success() {
    let client = sideko_rest_api::Client::default()
        .with_api_key_auth(&std::env::var("API_KEY").unwrap())
        .with_cookie_auth(&std::env::var("API_KEY").unwrap());
    let res = client
        .login_url(sideko_rest_api::LoginUrlRequest {
            ..Default::default()
        })
        .await;
    println!("{:?}", res);
    assert!(res.is_ok());
}
#[serial_test::serial]
#[tokio::test]
async fn test_cli_check_updates_200_generated_success() {
    let client = sideko_rest_api::Client::default()
        .with_api_key_auth(&std::env::var("API_KEY").unwrap())
        .with_cookie_auth(&std::env::var("API_KEY").unwrap());
    let res = client
        .cli_check_updates(sideko_rest_api::CliCheckUpdatesRequest {
            cli_version: "0.1.0".to_string(),
        })
        .await;
    println!("{:?}", res);
    assert!(res.is_ok());
}
#[serial_test::serial]
#[tokio::test]
async fn test_list_doc_projects_200_generated_success() {
    let client = sideko_rest_api::Client::default()
        .with_api_key_auth(&std::env::var("API_KEY").unwrap())
        .with_cookie_auth(&std::env::var("API_KEY").unwrap());
    let res = client.list_doc_projects().await;
    println!("{:?}", res);
    assert!(res.is_ok());
}
#[serial_test::serial]
#[tokio::test]
async fn test_get_doc_project_200_generated_success() {
    let client = sideko_rest_api::Client::default()
        .with_api_key_auth(&std::env::var("API_KEY").unwrap())
        .with_cookie_auth(&std::env::var("API_KEY").unwrap());
    let res = client
        .get_doc_project(sideko_rest_api::GetDocProjectRequest {
            doc_name: "my-project".to_string(),
        })
        .await;
    println!("{:?}", res);
    assert!(res.is_ok());
}
#[serial_test::serial]
#[tokio::test]
async fn test_list_deployments_200_generated_success() {
    let client = sideko_rest_api::Client::default()
        .with_api_key_auth(&std::env::var("API_KEY").unwrap())
        .with_cookie_auth(&std::env::var("API_KEY").unwrap());
    let res = client
        .list_deployments(sideko_rest_api::ListDeploymentsRequest {
            doc_name: "my-project".to_string(),
            ..Default::default()
        })
        .await;
    println!("{:?}", res);
    assert!(res.is_ok());
}
#[serial_test::serial]
#[tokio::test]
async fn test_get_deployment_200_generated_success() {
    let client = sideko_rest_api::Client::default()
        .with_api_key_auth(&std::env::var("API_KEY").unwrap())
        .with_cookie_auth(&std::env::var("API_KEY").unwrap());
    let res = client
        .get_deployment(sideko_rest_api::GetDeploymentRequest {
            doc_name: "my-project".to_string(),
            deployment_id: "3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a".to_string(),
        })
        .await;
    println!("{:?}", res);
    assert!(res.is_ok());
}
#[serial_test::serial]
#[tokio::test]
async fn test_check_preview_200_generated_success() {
    let client = sideko_rest_api::Client::default()
        .with_api_key_auth(&std::env::var("API_KEY").unwrap())
        .with_cookie_auth(&std::env::var("API_KEY").unwrap());
    let res = client
        .check_preview(sideko_rest_api::CheckPreviewRequest {
            doc_name: "my-project".to_string(),
        })
        .await;
    println!("{:?}", res);
    assert!(res.is_ok());
}
#[serial_test::serial]
#[tokio::test]
async fn test_get_doc_project_theme_200_generated_success() {
    let client = sideko_rest_api::Client::default()
        .with_api_key_auth(&std::env::var("API_KEY").unwrap())
        .with_cookie_auth(&std::env::var("API_KEY").unwrap());
    let res = client
        .get_doc_project_theme(sideko_rest_api::GetDocProjectThemeRequest {
            doc_name: "my-project".to_string(),
        })
        .await;
    println!("{:?}", res);
    assert!(res.is_ok());
}
#[serial_test::serial]
#[tokio::test]
async fn test_list_doc_versions_200_generated_success() {
    let client = sideko_rest_api::Client::default()
        .with_api_key_auth(&std::env::var("API_KEY").unwrap())
        .with_cookie_auth(&std::env::var("API_KEY").unwrap());
    let res = client
        .list_doc_versions(sideko_rest_api::ListDocVersionsRequest {
            doc_name: "my-project".to_string(),
        })
        .await;
    println!("{:?}", res);
    assert!(res.is_ok());
}
#[serial_test::serial]
#[tokio::test]
async fn test_get_doc_version_200_generated_success() {
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
    println!("{:?}", res);
    assert!(res.is_ok());
}
#[serial_test::serial]
#[tokio::test]
async fn test_list_guides_200_generated_success() {
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
    println!("{:?}", res);
    assert!(res.is_ok());
}
#[serial_test::serial]
#[tokio::test]
async fn test_get_guide_200_generated_success() {
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
    println!("{:?}", res);
    assert!(res.is_ok());
}
#[serial_test::serial]
#[tokio::test]
async fn test_get_guide_content_200_generated_success() {
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
    println!("{:?}", res);
    assert!(res.is_ok());
}
#[serial_test::serial]
#[tokio::test]
async fn test_get_organization_200_generated_success() {
    let client = sideko_rest_api::Client::default()
        .with_api_key_auth(&std::env::var("API_KEY").unwrap())
        .with_cookie_auth(&std::env::var("API_KEY").unwrap());
    let res = client.get_organization().await;
    println!("{:?}", res);
    assert!(res.is_ok());
}
#[serial_test::serial]
#[tokio::test]
async fn test_list_assets_200_generated_success() {
    let client = sideko_rest_api::Client::default()
        .with_api_key_auth(&std::env::var("API_KEY").unwrap())
        .with_cookie_auth(&std::env::var("API_KEY").unwrap());
    let res = client
        .list_assets(sideko_rest_api::ListAssetsRequest {
            ..Default::default()
        })
        .await;
    println!("{:?}", res);
    assert!(res.is_ok());
}
#[serial_test::serial]
#[tokio::test]
async fn test_get_organization_theme_200_generated_success() {
    let client = sideko_rest_api::Client::default()
        .with_api_key_auth(&std::env::var("API_KEY").unwrap())
        .with_cookie_auth(&std::env::var("API_KEY").unwrap());
    let res = client.get_organization_theme().await;
    println!("{:?}", res);
    assert!(res.is_ok());
}
#[serial_test::serial]
#[tokio::test]
async fn test_get_current_user_200_generated_success() {
    let client = sideko_rest_api::Client::default()
        .with_api_key_auth(&std::env::var("API_KEY").unwrap())
        .with_cookie_auth(&std::env::var("API_KEY").unwrap());
    let res = client.get_current_user().await;
    println!("{:?}", res);
    assert!(res.is_ok());
}
#[serial_test::serial]
#[tokio::test]
async fn test_get_api_key_200_generated_success() {
    let client = sideko_rest_api::Client::default()
        .with_api_key_auth(&std::env::var("API_KEY").unwrap())
        .with_cookie_auth(&std::env::var("API_KEY").unwrap());
    let res = client.get_api_key().await;
    println!("{:?}", res);
    assert!(res.is_ok());
}
#[serial_test::serial]
#[tokio::test]
async fn test_list_service_accounts_200_generated_success() {
    let client = sideko_rest_api::Client::default()
        .with_api_key_auth(&std::env::var("API_KEY").unwrap())
        .with_cookie_auth(&std::env::var("API_KEY").unwrap());
    let res = client.list_service_accounts().await;
    println!("{:?}", res);
    assert!(res.is_ok());
}
#[serial_test::serial]
#[tokio::test]
async fn test_get_service_account_200_generated_success() {
    let client = sideko_rest_api::Client::default()
        .with_api_key_auth(&std::env::var("API_KEY").unwrap())
        .with_cookie_auth(&std::env::var("API_KEY").unwrap());
    let res = client
        .get_service_account(sideko_rest_api::GetServiceAccountRequest {
            id: "3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a".to_string(),
        })
        .await;
    println!("{:?}", res);
    assert!(res.is_ok());
}
#[serial_test::serial]
#[tokio::test]
async fn test_update_api_link_200_success_default() {
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
    println!("{:?}", res);
    assert!(res.is_ok());
}
#[serial_test::serial]
#[tokio::test]
async fn test_update_api_link_group_200_success_default() {
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
    println!("{:?}", res);
    assert!(res.is_ok());
}
#[serial_test::serial]
#[tokio::test]
async fn test_update_doc_project_200_success_default() {
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
    println!("{:?}", res);
    assert!(res.is_ok());
}
#[serial_test::serial]
#[tokio::test]
async fn test_update_guide_200_success_default() {
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
    println!("{:?}", res);
    assert!(res.is_ok());
}
#[serial_test::serial]
#[tokio::test]
async fn test_update_asset_200_success_default() {
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
    println!("{:?}", res);
    assert!(res.is_ok());
}
#[serial_test::serial]
#[tokio::test]
async fn test_create_api_link_201_success_default() {
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
    println!("{:?}", res);
    assert!(res.is_ok());
}
#[serial_test::serial]
#[tokio::test]
async fn test_reorder_api_links_200_success_default() {
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
    println!("{:?}", res);
    assert!(res.is_ok());
}
#[serial_test::serial]
#[tokio::test]
async fn test_create_api_link_group_201_success_default() {
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
    println!("{:?}", res);
    assert!(res.is_ok());
}
#[serial_test::serial]
#[tokio::test]
async fn test_create_doc_project_201_success_default() {
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
    println!("{:?}", res);
    assert!(res.is_ok());
}
#[serial_test::serial]
#[tokio::test]
async fn test_trigger_deployment_201_success_default() {
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
    println!("{:?}", res);
    assert!(res.is_ok());
}
#[serial_test::serial]
#[tokio::test]
async fn test_create_guide_201_success_default() {
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
    println!("{:?}", res);
    assert!(res.is_ok());
}
#[serial_test::serial]
#[tokio::test]
async fn test_reorder_guides_200_success_default() {
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
    println!("{:?}", res);
    assert!(res.is_ok());
}
#[serial_test::serial]
#[tokio::test]
async fn test_create_organization_201_success_default() {
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
    println!("{:?}", res);
    assert!(res.is_ok());
}
#[serial_test::serial]
#[tokio::test]
async fn test_upload_assets_201_success_default() {
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
    println!("{:?}", res);
    assert!(res.is_ok());
}
#[serial_test::serial]
#[tokio::test]
async fn test_invite_user_202_success_default() {
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
    println!("{:?}", res);
    assert!(res.is_ok());
}
#[serial_test::serial]
#[tokio::test]
async fn test_create_service_account_200_success_default() {
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
    println!("{:?}", res);
    assert!(res.is_ok());
}
#[serial_test::serial]
#[tokio::test]
async fn test_vercel_webhook_202_success_default() {
    let client = sideko_rest_api::Client::default()
        .with_api_key_auth(&std::env::var("API_KEY").unwrap())
        .with_cookie_auth(&std::env::var("API_KEY").unwrap());
    let res = client
        .vercel_webhook(sideko_rest_api::VercelWebhookRequest {
            data: serde_json::json!({}),
        })
        .await;
    println!("{:?}", res);
    assert!(res.is_ok());
}
#[serial_test::serial]
#[tokio::test]
async fn test_update_doc_project_theme_200_success_default() {
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
    println!("{:?}", res);
    assert!(res.is_ok());
}
#[serial_test::serial]
#[tokio::test]
async fn test_update_organization_theme_200_success_default() {
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
    println!("{:?}", res);
    assert!(res.is_ok());
}
