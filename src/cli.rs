use std::fmt;
use std::net;
use std::path::PathBuf;
use std::str::FromStr;

use structopt::clap::Shell;
use structopt::StructOpt;

#[derive(Debug)]
pub enum ColorMode {
    Always,
    Never,
    Auto,
}

impl Default for ColorMode {
    fn default() -> Self {
        ColorMode::Auto
    }
}

impl FromStr for ColorMode {
    type Err = &'static str;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "always" => Ok(Self::Always),
            "never" => Ok(Self::Never),
            "auto" => Ok(Self::Auto),
            _ => Err("valid values: always, never, auto"),
        }
    }
}

impl fmt::Display for ColorMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Always => write!(f, "always"),
            Self::Never => write!(f, "never"),
            Self::Auto => write!(f, "auto"),
        }
    }
}

impl ColorMode {
    pub fn variants() -> [&'static str; 3] {
        ["always", "never", "auto"]
    }
}

#[derive(Debug)]
pub enum JsonFormat {
    Pretty,
    Compact,
}

impl FromStr for JsonFormat {
    type Err = &'static str;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "pretty" => Ok(Self::Pretty),
            "compact" => Ok(Self::Compact),
            _ => Err("valid values: pretty, compact"),
        }
    }
}

impl fmt::Display for JsonFormat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Pretty => write!(f, "pretty"),
            Self::Compact => write!(f, "compact"),
        }
    }
}

impl JsonFormat {
    pub fn variants() -> [&'static str; 2] {
        ["pretty", "compact"]
    }
}

#[derive(Debug, StructOpt)]
#[structopt(name = "ipinfo", global_setting = structopt::clap::AppSettings::ColoredHelp)]
/// Get IP details from IPinfo.io
pub struct Args {
    /// Color mode
    #[structopt(
        short = "c",
        long = "color",
        default_value = "auto",
        possible_values = &ColorMode::variants(),
    )]
    pub color: ColorMode,
    /// Generates a completion file
    #[structopt(
        long = "completion",
        possible_values = &Shell::variants(),
    )]
    pub completion: Option<Shell>,
    /// Formatting
    #[structopt(
        short = "f",
        long = "format",
        default_value = "pretty",
        possible_values = &JsonFormat::variants(),
    )]
    pub format: JsonFormat,
    /// Output file
    #[structopt(short = "o", long = "output", parse(from_os_str))]
    pub output: Option<PathBuf>,
    /// API token
    #[structopt(short = "t", long = "token", env = "IPINFO_TOKEN")]
    pub token: Option<String>,
    /// IP address
    pub ip: Option<net::IpAddr>,
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::{ColorMode, JsonFormat};

    #[test]
    fn test_color_mode() {
        for variant in &ColorMode::variants() {
            assert_eq!(
                format!("{}", ColorMode::from_str(variant).unwrap()),
                *variant
            );
        }
    }

    #[test]
    fn test_json_format() {
        for variant in &JsonFormat::variants() {
            assert_eq!(
                format!("{}", JsonFormat::from_str(variant).unwrap()),
                *variant
            );
        }
    }
}
