use log::{error, info};
use proto::{MessageRequest, MessageResponse, message_service_server::MessageService};
use tokio::sync::{broadcast, mpsc};
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Result};

pub struct MessageHandler {
    pub sender: broadcast::Sender<MessageRequest>,
}

impl MessageHandler {
    pub fn new() -> Self {
        let (sender, _receiver) = broadcast::channel(10);

        Self { sender }
    }
}

#[tonic::async_trait]
impl MessageService for MessageHandler {
    type ListenStream = ReceiverStream<Result<MessageResponse>>;

    async fn send(&self, request: Request<MessageRequest>) -> Result<Response<()>> {
        let message = request.into_inner();
        info!("{}", message.content);

        self.sender.send(message).expect("Fail brodcast message");

        Ok(Response::new(()))
    }

    async fn listen(&self, _request: Request<()>) -> Result<Response<Self::ListenStream>> {
        let (sender, receiver) = mpsc::channel(4);
        let mut chat_receiver = self.sender.subscribe();
        info!("Client connected");

        tokio::spawn(async move {
            while let Ok(message) = chat_receiver.recv().await {
                let response = sender
                    .send(Ok(MessageResponse {
                        content: message.content,
                        timestamp: 0,
                    }))
                    .await;

                if let Err(err) = response {
                    error!("{:?}", err);
                }
            }
        });

        Ok(Response::new(ReceiverStream::new(receiver)))
    }
}
