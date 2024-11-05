#[derive(Clone, Debug)]
pub struct StatelessClient {
    base_client: crate::core::base_client::BaseClient,
}
impl StatelessClient {
    pub(crate) fn new(base_client: crate::core::base_client::BaseClient) -> Self {
        Self { base_client }
    }
    pub fn generate_sdk(
        &self,
    ) -> crate::resources::stateless::generate_sdk::resource_client::GenerateSdkClient {
        crate::resources::stateless::generate_sdk::resource_client::GenerateSdkClient::new(
            self.base_client.clone(),
        )
    }
}
