use anyhow::Result;
use proto::message_service_client::MessageServiceClient;
use tonic::transport::Channel;

use crate::config::Config;

pub struct Gateway {
    pub message_service: MessageServiceClient<Channel>,
}

impl Gateway {
    pub async fn new() -> Result<Self> {
        let config = Config::get();

        let channel = Channel::from_shared(config.gateway_url.clone())?
            .connect()
            .await?;

        let message_service = MessageServiceClient::new(channel);

        Ok(Gateway { message_service })
    }
}
