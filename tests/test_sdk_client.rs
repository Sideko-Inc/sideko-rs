#[serial_test::serial]
#[tokio::test]
async fn test_list_200_generated_success() {
    let client = sideko_rest_api::Client::default()
        .with_api_key_auth(&std::env::var("API_KEY").unwrap())
        .with_cookie_auth(&std::env::var("API_KEY").unwrap());
    let res = client
        .sdk()
        .list(sideko_rest_api::resources::sdk::ListRequest {
            ..Default::default()
        })
        .await;
    println!("{:?}", res);
    assert!(res.is_ok());
}
#[serial_test::serial]
#[tokio::test]
async fn test_generate_201_success_default() {
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
    println!("{:?}", res);
    assert!(res.is_ok());
}
#[serial_test::serial]
#[tokio::test]
async fn test_update_201_success_default() {
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
    println!("{:?}", res);
    assert!(res.is_ok());
}
#[serial_test::serial]
#[tokio::test]
async fn test_generate_stateless_201_success_default() {
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
    println!("{:?}", res);
    assert!(res.is_ok());
}
