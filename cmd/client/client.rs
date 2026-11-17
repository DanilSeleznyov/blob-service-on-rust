mod cli;
use blob_service_on_rust::proto::{
    Blob, CreateBlobRequest, DeleteBlobRequest, GetBlobRequest, ListBlobsRequest,
    UpdateBlobRequest, blob_service_client::BlobServiceClient,
};
use clap::Parser;
use cli::{Cli, Command, CreateArgs, DeleteArgs, GetArgs, ListArgs, UpdateArgs};
use std::{error::Error, ffi::OsStr, fs, io, path::Path};
use tonic::transport::Channel;

const GRPC_ADDR: &str = "http://localhost:50051";
const EVENT_CREATED: &str = "CREATED";
const EVENT_FOUND: &str = "FOUND";
const EVENT_UPDATED: &str = "UPDATED";
const EVENT_DELETED: &str = "DELETED";

fn print_blob(blob: &Blob, event: &str) {
    println!(
        r#"
========== BLOB {} ==========
==> Name: {}
==> Size: {} B
==> ID: {}
==> Type: {}
==> Hash: {}
==> Created: {:?}
==> Updated: {:?}
==> Tags: {:?}
==> Owner: {}
==> Access: {}
================================
            "#,
        event,
        blob.name,
        blob.size,
        blob.id,
        blob.content_type,
        blob.sha256_hash,
        blob.created_at,
        blob.updated_at,
        blob.tags,
        blob.owner_id,
        blob.access_level
    );
}

async fn run_create(
    client: &mut BlobServiceClient<Channel>,
    args: CreateArgs,
) -> Result<(), Box<dyn Error>> {
    let data = fs::read(&args.file)?;

    let name = Path::new(&args.file)
        .file_name()
        .and_then(OsStr::to_str)
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "invalid file name"))?
        .to_owned();

    let content_type = mime_guess::from_path(&args.file)
        .first_or_octet_stream()
        .to_string();

    let request = CreateBlobRequest {
        name,
        content_type,
        data,
        tags: args.tags,
    };
    let response = client.create_blob(request).await?.into_inner();

    let blob = response.blob.ok_or_else(|| {
        io::Error::new(io::ErrorKind::InvalidData, "server returned an empty blob")
    })?;

    print_blob(&blob, EVENT_CREATED);

    Ok(())
}

async fn run_get(
    client: &mut BlobServiceClient<Channel>,
    args: GetArgs,
) -> Result<(), Box<dyn Error>> {
    let request = GetBlobRequest { id: args.id };

    let response = client.get_blob(request).await?.into_inner();

    let blob = response
        .blob
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "blob not found"))?;

    if let Some(path) = args.out {
        fs::write(&path, &blob.data)?;
        println!("Saved {} bytes to {}", blob.data.len(), path.display());
    } else {
        print_blob(&blob, EVENT_FOUND);
    }

    Ok(())
}

async fn run_update(
    client: &mut BlobServiceClient<Channel>,
    args: UpdateArgs,
) -> Result<(), Box<dyn Error>> {
    let data = fs::read(&args.file)?;
    let name = Path::new(&args.file)
        .file_name()
        .and_then(OsStr::to_str)
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "invalid file name"))?
        .to_owned();
    let content_type = mime_guess::from_path(&args.file)
        .first_or_octet_stream()
        .to_string();

    let response = client
        .update_blob(UpdateBlobRequest {
            id: args.id,
            name,
            content_type,
            data,
            tags: args.tags,
            metadata: Default::default(),
            access_level: String::new(),
        })
        .await?
        .into_inner();
    let blob = response.blob.ok_or_else(|| {
        io::Error::new(io::ErrorKind::InvalidData, "server returned an empty blob")
    })?;

    print_blob(&blob, EVENT_UPDATED);
    Ok(())
}

async fn run_delete(
    client: &mut BlobServiceClient<Channel>,
    args: DeleteArgs,
) -> Result<(), Box<dyn Error>> {
    let response = client
        .delete_blob(DeleteBlobRequest { id: args.id })
        .await?
        .into_inner();
    let blob = response
        .blob
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "blob not found"))?;

    print_blob(&blob, EVENT_DELETED);
    Ok(())
}

async fn run_list(
    client: &mut BlobServiceClient<Channel>,
    args: ListArgs,
) -> Result<(), Box<dyn Error>> {
    let limit = i32::try_from(args.limit)?;
    let response = client
        .list_blobs(ListBlobsRequest {
            name_filter: args.name.unwrap_or_default(),
            content_type_filter: args.content_type.unwrap_or_default(),
            tag_filters: args.tags,
            limit,
        })
        .await?
        .into_inner();

    for blob in &response.blobs {
        println!(
            "{}  {}  {} B  {:?}",
            blob.id, blob.name, blob.size, blob.tags
        );
    }
    println!("Total: {}", response.blobs.len());
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    let mut client = BlobServiceClient::connect(GRPC_ADDR).await?;

    match cli.command {
        Command::Create(args) => run_create(&mut client, args).await?,
        Command::Get(args) => run_get(&mut client, args).await?,
        Command::List(args) => run_list(&mut client, args).await?,
        Command::Update(args) => run_update(&mut client, args).await?,
        Command::Delete(args) => run_delete(&mut client, args).await?,
    }

    Ok(())
}
