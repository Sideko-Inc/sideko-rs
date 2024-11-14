#[serial_test::serial]
#[tokio::test]
async fn test_delete_204_generated_success() {
    let client = sideko_rest_api::Client::default()
        .with_api_key_auth(&std::env::var("API_KEY").unwrap())
        .with_cookie_auth(&std::env::var("API_KEY").unwrap());
    let res = client
        .role()
        .delete(sideko_rest_api::resources::role::DeleteRequest {
            id: "3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a".to_string(),
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
        .role()
        .list(sideko_rest_api::resources::role::ListRequest {
            ..Default::default()
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
    println!("{:?}", res);
    assert!(res.is_ok());
}
