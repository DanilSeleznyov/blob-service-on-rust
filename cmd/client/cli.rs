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
    pub file: PathBuf,

    #[arg(long, value_delimiter = ',')]
    pub tags: Vec<String>,
}

#[derive(Args)]
pub struct DeleteArgs {
    #[arg(long)]
    pub id: String,
}
