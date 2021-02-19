use std::io;
use std::net;

use colored_json::{ColoredFormatter, CompactFormatter, PrettyFormatter};
use serde::Serialize;
use structopt::clap::Shell;
use structopt::StructOpt;

use crate::cli::{self, ColorMode, JsonFormat};
use crate::error::Result;
use crate::ipinfo::Client;

fn generate_completion(shell: Shell) {
    cli::Args::clap().gen_completions_to("ipinfo", shell, &mut io::stdout());
}

fn build_client(token: Option<String>) -> Client {
    let mut client_builder = Client::builder();
    if let Some(token) = token {
        client_builder = client_builder.with_token(token);
    }
    client_builder.build()
}

async fn get_info(client: &Client, ip: Option<net::IpAddr>) -> Result<serde_json::Value> {
    if let Some(ip) = ip {
        client.get_ip(&ip).await
    } else {
        client.get().await
    }
}

fn use_color(color: &ColorMode) -> bool {
    match color {
        ColorMode::Always => true,
        ColorMode::Never => false,
        ColorMode::Auto => colored_json::ColorMode::Auto(colored_json::Output::StdOut).use_color(),
    }
}

fn serialize_info<F: serde_json::ser::Formatter, W: io::Write>(
    info: &serde_json::Value,
    formatter: F,
    output: &mut W,
) -> Result<()> {
    let mut serializer = serde_json::Serializer::with_formatter(output, formatter);
    info.serialize(&mut serializer)?;
    writeln!(serializer.into_inner())?;
    Ok(())
}

fn print_info(info: &serde_json::Value, color: &ColorMode, format: &JsonFormat) -> Result<()> {
    let output = &mut io::stdout();
    match (use_color(color), format) {
        (false, JsonFormat::Compact) => {
            let formatter = CompactFormatter {};
            serialize_info(info, formatter, output)
        }
        (false, JsonFormat::Pretty) => {
            let formatter = PrettyFormatter::new();
            serialize_info(info, formatter, output)
        }
        (true, JsonFormat::Compact) => {
            let formatter = ColoredFormatter::new(CompactFormatter {});
            serialize_info(info, formatter, output)
        }
        (true, JsonFormat::Pretty) => {
            let formatter = ColoredFormatter::new(PrettyFormatter::new());
            serialize_info(info, formatter, output)
        }
    }
}

pub async fn main(args: cli::Args) -> Result<()> {
    if let Some(shell) = args.completion {
        generate_completion(shell);
    } else {
        let client = build_client(args.token);
        let info = get_info(&client, args.ip).await?;
        print_info(&info, &args.color, &args.format)?;
    }
    Ok(())
}
