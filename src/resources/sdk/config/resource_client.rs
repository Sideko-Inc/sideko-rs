#[derive(Clone, Debug)]
pub struct ConfigClient {
    base_client: crate::core::base_client::BaseClient,
}
impl ConfigClient {
    pub(crate) fn new(base_client: crate::core::base_client::BaseClient) -> Self {
        Self { base_client }
    }
    pub fn init(
        &self,
    ) -> crate::resources::sdk::config::init::resource_client::InitClient {
        crate::resources::sdk::config::init::resource_client::InitClient::new(
            self.base_client.clone(),
        )
    }
    pub fn sync(
        &self,
    ) -> crate::resources::sdk::config::sync::resource_client::SyncClient {
        crate::resources::sdk::config::sync::resource_client::SyncClient::new(
            self.base_client.clone(),
        )
    }
}
