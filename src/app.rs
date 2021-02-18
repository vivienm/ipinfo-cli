use std::io;

use colored_json::{ColoredFormatter, CompactFormatter, PrettyFormatter};
use serde::Serialize;

use crate::cli::{self, ColorMode, JsonFormat};
use crate::error::Result;
use crate::ipinfo::Client;

fn use_color(color: &ColorMode) -> bool {
    match color {
        ColorMode::Always => true,
        ColorMode::Never => false,
        ColorMode::Auto => colored_json::ColorMode::Auto(colored_json::Output::StdOut).use_color(),
    }
}

fn serialize<F: serde_json::ser::Formatter, W: io::Write>(
    value: &serde_json::Value,
    formatter: F,
    output: &mut W,
) -> Result<()> {
    let mut serializer = serde_json::Serializer::with_formatter(output, formatter);
    value.serialize(&mut serializer)?;
    writeln!(serializer.into_inner())?;
    Ok(())
}

pub async fn main(args: cli::Args) -> Result<()> {
    // Build the client.
    let mut client_builder = Client::builder();
    if let Some(token) = args.token {
        client_builder = client_builder.with_token(token);
    }
    let client = client_builder.build();

    // Run the query.
    let ip_info = if let Some(ip) = args.ip {
        client.get_ip(&ip).await
    } else {
        client.get().await
    }?;

    // Print results.
    let output = &mut io::stdout();
    match (use_color(&args.color), args.format) {
        (false, JsonFormat::Compact) => {
            let formatter = CompactFormatter {};
            serialize(&ip_info, formatter, output)
        }
        (false, JsonFormat::Pretty) => {
            let formatter = PrettyFormatter::new();
            serialize(&ip_info, formatter, output)
        }
        (true, JsonFormat::Compact) => {
            let formatter = ColoredFormatter::new(CompactFormatter {});
            serialize(&ip_info, formatter, output)
        }
        (true, JsonFormat::Pretty) => {
            let formatter = ColoredFormatter::new(PrettyFormatter::new());
            serialize(&ip_info, formatter, output)
        }
    }
}
