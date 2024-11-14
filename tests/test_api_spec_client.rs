#[serial_test::serial]
#[tokio::test]
async fn test_delete_204_generated_success() {
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
    println!("{:?}", res);
    assert!(res.is_ok());
}
#[serial_test::serial]
#[tokio::test]
async fn test_list_200_generated_success() {
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
    println!("{:?}", res);
    assert!(res.is_ok());
}
#[serial_test::serial]
#[tokio::test]
async fn test_get_200_generated_success() {
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
    println!("{:?}", res);
    assert!(res.is_ok());
}
#[serial_test::serial]
#[tokio::test]
async fn test_patch_200_success_default() {
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
    println!("{:?}", res);
    assert!(res.is_ok());
}
#[serial_test::serial]
#[tokio::test]
async fn test_create_201_success_default() {
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
    println!("{:?}", res);
    assert!(res.is_ok());
}
