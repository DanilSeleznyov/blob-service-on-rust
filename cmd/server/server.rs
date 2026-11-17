use blob_service_on_rust::{
    internal::{config, db, repo::BlobRepository, server::BlobServer},
    proto::blob_service_server::BlobServiceServer,
};
use tonic::transport::Server;

const MAX_MESSAGE_SIZE: usize = 32 * 1024 * 1024;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cfg = config::load()?;
    let pool = db::connect(&cfg.database).await?;
    let server = BlobServer::new(BlobRepository::new(pool));
    let addr = format!("{}:{}", cfg.server.host, cfg.server.port).parse()?;

    println!("gRPC server is listening on {addr}");

    Server::builder()
        .add_service(
            BlobServiceServer::new(server)
                .max_decoding_message_size(MAX_MESSAGE_SIZE)
                .max_encoding_message_size(MAX_MESSAGE_SIZE),
        )
        .serve(addr)
        .await?;

    Ok(())
}
