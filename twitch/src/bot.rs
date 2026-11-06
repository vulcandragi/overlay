use std::sync::Arc;

use anyhow::Result;
use log::{error, info, warn};
use proto::Message;
use tokio::{sync::mpsc::UnboundedReceiver, task::JoinHandle};
use tonic::Request;
use twitch_irc::{
    ClientConfig, SecureTCPTransport, TwitchIRCClient, login::StaticLoginCredentials,
    message::ServerMessage,
};

use crate::{config::Config, gateway::Gateway};

pub type TwitchClient = TwitchIRCClient<SecureTCPTransport, StaticLoginCredentials>;

pub struct Bot {
    pub client: Arc<TwitchClient>,
    pub channel_message: UnboundedReceiver<ServerMessage>,
    pub gateway: Gateway,
}

impl Bot {
    pub async fn new() -> Result<Self> {
        let config = Config::get();
        let login_credentials = StaticLoginCredentials::new(
            config.bot_username.to_owned(),
            Some(config.bot_token.to_owned()),
        );

        let client_config = ClientConfig::new_simple(login_credentials);
        let (channel_message, client) = TwitchClient::new(client_config);

        client.join(config.channel_to_join.to_owned())?;

        let gateway = Gateway::new().await?;

        Ok(Bot {
            client: Arc::new(client),
            channel_message,
            gateway,
        })
    }

    pub fn handle_messages(mut self) -> JoinHandle<()> {
        tokio::spawn(async move {
            while let Some(message) = self.channel_message.recv().await {
                let ServerMessage::Privmsg(message) = message else {
                    continue;
                };

                if message.sender.name == "StreamElements" {
                    continue;
                }

                info!("Mensagem recebida do chat: {}", message.message_text);

                if message.message_text.starts_with("!ping") {
                    self.client
                        .say(message.channel_login, "pong".to_string())
                        .await
                        .unwrap();
                    continue;
                }

                let response = self
                    .gateway
                    .message_service
                    .send(Request::new(Message {
                        message_id: message.message_id.to_string(),
                        content: message.message_text,
                        name: message.sender.name.to_string(),
                        name_color: message.name_color.unwrap().to_string(),
                    }))
                    .await;

                if let Err(e) = response {
                    error!("Fail send message to gRPC: {:?}", e);
                    continue;
                }
            }

            warn!("O canal de mensagens da Twitch foi fechado (conexão encerrada).");
        })
    }
}
