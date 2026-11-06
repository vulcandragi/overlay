use std::error::Error;

use log::LevelFilter;
use proto::message_service_server::MessageServiceServer;
use tonic::transport::Server;
use tonic_web::GrpcWebLayer;
use tower_http::cors::{AllowHeaders, AllowMethods, AllowOrigin, CorsLayer};

use crate::services::message::MessageHandler;

mod services;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::builder()
        .filter_level(LevelFilter::Debug)
        .init();

    let address = "0.0.0.0:50051".parse()?;

    Server::builder()
        .accept_http1(true)
        .layer(
            CorsLayer::new()
                .allow_origin(AllowOrigin::mirror_request())
                .allow_headers(AllowHeaders::mirror_request())
                .allow_methods(AllowMethods::mirror_request()),
        )
        .layer(GrpcWebLayer::new())
        .add_service(MessageServiceServer::new(MessageHandler::new()))
        .serve(address)
        .await?;
    Ok(())
}
