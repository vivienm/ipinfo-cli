use std::env;
use std::fs;

use structopt::clap::Shell;
use structopt::StructOpt;

#[path = "src/cli.rs"]
mod cli;

fn main() {
    let output_dir = env::var_os("OUT_DIR").unwrap();
    fs::create_dir_all(&output_dir).unwrap();

    let mut clap = cli::Args::clap();
    for shell in &[Shell::Bash, Shell::Fish, Shell::Zsh] {
        clap.gen_completions("ipinfo", *shell, &output_dir);
    }
}
