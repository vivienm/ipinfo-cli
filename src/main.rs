use structopt::StructOpt;

mod app;
mod cli;
pub mod error;
pub mod ipinfo;
use crate::error::Result;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    app::main(cli::Args::from_args()).await
}
