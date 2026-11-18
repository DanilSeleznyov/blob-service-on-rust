use clap::{Args, Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "client", about = "Blob service client")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    Create(CreateArgs),
    Get(GetArgs),
    List(ListArgs),
    Update(UpdateArgs),
    Delete(DeleteArgs),
}

#[derive(Args)]
pub struct CreateArgs {
    #[arg(long)]
    pub file: PathBuf,

    #[arg(long, value_delimiter = ',')]
    pub tags: Vec<String>,

    #[arg(long = "metadata", value_delimiter = ',', value_parser = parse_key_val)]
    pub metadata: Vec<(String, String)>,

    #[arg(long = "owner-id")]
    pub owner_id: Option<String>,

    #[arg(long = "access-level")]
    pub access_level: Option<String>,
}

#[derive(Args)]
pub struct GetArgs {
    #[arg(long)]
    pub id: String,

    #[arg(long)]
    pub out: Option<PathBuf>,
}

#[derive(Args)]
pub struct ListArgs {
    #[arg(long)]
    pub name: Option<String>,

    #[arg(long = "content-type")]
    pub content_type: Option<String>,

    #[arg(long, value_delimiter = ',')]
    pub tags: Vec<String>,

    #[arg(long, default_value_t = 10)]
    pub limit: u32,
}

#[derive(Args)]
pub struct UpdateArgs {
    #[arg(long)]
    pub id: String,

    #[arg(long)]
    pub file: Option<PathBuf>,

    #[arg(long)]
    pub name: Option<String>,

    #[arg(long = "content-type")]
    pub content_type: Option<String>,

    #[arg(long, value_delimiter = ',')]
    pub tags: Vec<String>,

    #[arg(long = "metadata", value_delimiter = ',', value_parser = parse_key_val)]
    pub metadata: Vec<(String, String)>,
}

#[derive(Args)]
pub struct DeleteArgs {
    #[arg(long)]
    pub id: String,
}

fn parse_key_val(s: &str) -> Result<(String, String), String> {
    match s.split_once('=') {
        Some((key, value)) if !key.is_empty() => Ok((key.to_owned(), value.to_owned())),
        _ => Err(format!("expected KEY=VALUE, got `{s}`")),
    }
}
