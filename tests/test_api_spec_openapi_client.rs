#[serial_test::serial]
#[tokio::test]
async fn test_get_openapi_200_generated_success() {
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
    println!("{:?}", res);
    assert!(res.is_ok());
}
