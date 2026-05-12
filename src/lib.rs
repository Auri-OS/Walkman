pub mod config;
pub mod engine;
pub mod reporter;
pub mod traits;

const GREEN: &str = "\x1b[32m";
const RED: &str = "\x1b[31m";
const GREY: &str = "\x1b[90m";
const RESET: &str = "\x1b[0m";

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    pub path: String,

    #[arg(long, default_value_t = false)]
    pub nui: bool,

    #[arg(long, default_value_t = false)]
    pub verbose: bool,
}
