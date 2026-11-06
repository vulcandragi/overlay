use proto::message_service_client::MessageServiceClient;
use tonic_web_wasm_client::Client;

use crate::config::Config;

pub struct MessageService {
    pub client: MessageServiceClient<Client>,
}

impl MessageService {
    pub fn new() -> Self {
        let config = Config::get();
        let transport = Client::new(config.core_url.to_string());
        let client = MessageServiceClient::new(transport);

        Self { client }
    }
}