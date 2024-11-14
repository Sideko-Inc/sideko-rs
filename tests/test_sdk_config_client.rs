#[serial_test::serial]
#[tokio::test]
async fn test_init_200_success_default() {
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
    println!("{:?}", res);
    assert!(res.is_ok());
}
#[serial_test::serial]
#[tokio::test]
async fn test_sync_200_success_default() {
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
    println!("{:?}", res);
    assert!(res.is_ok());
}
