use proto::message_service_client::MessageServiceClient;
use tonic_web_wasm_client::Client;

pub struct MessageService {
    pub client: MessageServiceClient<Client>,
}

impl MessageService {
    pub fn new() -> Self {
        let transport = Client::new("http://localhost:50051".into());
        let client = MessageServiceClient::new(transport);

        Self { client }
    }
}