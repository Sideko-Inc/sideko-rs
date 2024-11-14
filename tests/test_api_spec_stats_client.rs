#[serial_test::serial]
#[tokio::test]
async fn test_get_stats_200_generated_success() {
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
    println!("{:?}", res);
    assert!(res.is_ok());
}
