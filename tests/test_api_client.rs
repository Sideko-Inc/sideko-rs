#[serial_test::serial]
#[tokio::test]
async fn test_delete_204_generated_success() {
    let client = sideko_rest_api::Client::default()
        .with_api_key_auth(&std::env::var("API_KEY").unwrap())
        .with_cookie_auth(&std::env::var("API_KEY").unwrap());
    let res = client
        .api()
        .delete(sideko_rest_api::resources::api::DeleteRequest {
            api_name: "my-project".to_string(),
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
    let res = client.api().list().await;
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
        .get(sideko_rest_api::resources::api::GetRequest {
            api_name: "my-project".to_string(),
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
        .patch(sideko_rest_api::resources::api::PatchRequest {
            api_name: "my-project".to_string(),
            data: sideko_rest_api::models::UpdateApi {
                name: Some("my-new-api-name".to_string()),
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
        .create(sideko_rest_api::resources::api::CreateRequest {
            data: sideko_rest_api::models::NewApi {
                name: "my-api-spec".to_string(),
            },
        })
        .await;
    println!("{:?}", res);
    assert!(res.is_ok());
}
