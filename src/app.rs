use std::fs;
use std::io;
use std::net;
use std::path::Path;

use colored_json::{ColoredFormatter, CompactFormatter, PrettyFormatter};
use serde::Serialize;
use structopt::clap::Shell;
use structopt::StructOpt;

use crate::cli::{self, ColorMode, JsonFormat};
use crate::error::Result;
use crate::ipinfo::{Client, IpInfo};

fn open_output<P: AsRef<Path>>(path: Option<P>) -> Result<Box<dyn io::Write>> {
    Ok(match path {
        None => Box::new(io::stdout()),
        Some(path) => Box::new(fs::File::create(path)?),
    })
}

fn generate_completion<W: io::Write>(shell: Shell, output: &mut W) {
    cli::Args::clap().gen_completions_to("ipinfo", shell, output);
}

fn build_client(token: Option<String>) -> Client {
    let mut client_builder = Client::builder();
    if let Some(token) = token {
        client_builder = client_builder.with_token(token);
    }
    client_builder.build()
}

async fn get_info(client: &Client, ip: Option<net::IpAddr>) -> Result<IpInfo> {
    if let Some(ip) = ip {
        client.get_ip(&ip).await
    } else {
        client.get().await
    }
}

fn use_color(color: &ColorMode, is_stdout: bool) -> bool {
    match color {
        ColorMode::Always => true,
        ColorMode::Never => false,
        ColorMode::Auto if !is_stdout => false,
        ColorMode::Auto => colored_json::ColorMode::Auto(colored_json::Output::StdOut).use_color(),
    }
}

fn serialize_info<F: serde_json::ser::Formatter, W: io::Write>(
    info: &IpInfo,
    formatter: F,
    output: &mut W,
) -> Result<()> {
    let mut serializer = serde_json::Serializer::with_formatter(output, formatter);
    info.as_value().serialize(&mut serializer)?;
    writeln!(serializer.into_inner())?;
    Ok(())
}

fn print_info<W: io::Write>(
    info: &IpInfo,
    color: bool,
    format: &JsonFormat,
    output: &mut W,
) -> Result<()> {
    match (color, format) {
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
    let mut output = open_output(args.output.as_ref())?;
    if let Some(shell) = args.completion {
        generate_completion(shell, &mut output);
    } else {
        let client = build_client(args.token);
        print_info(
            &get_info(&client, args.ip).await?,
            use_color(&args.color, args.output.is_none()),
            &args.format,
            &mut output,
        )?;
    }
    Ok(())
}
