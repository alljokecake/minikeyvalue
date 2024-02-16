pub mod rebalance;
pub mod rebuild;
pub mod server;

use std::path::PathBuf;

use anyhow::Result;
use clap::{Parser, Subcommand};

static NAME: &str = "mkv";
static USAGE: &str = "mkv --volumes <VOLUMES> --db <PATH> [OPTIONS] <COMMAND>";

#[derive(Parser, Debug)]
#[clap(name = NAME, override_usage = USAGE, version, about, long_about = None)]
pub struct Options {
    /// Path to database
    #[clap(long = "db", value_name = "PATH", required = true)]
    pub pdb: PathBuf,

    /// Fallback server for missing keys
    #[clap(long)]
    pub fallback: Option<String>,

    /// Calculate and stor MD5 checksum of values
    #[clap(long, hide = true, default_value_t = true)]
    pub md5sum: bool,

    /// Port for the server to listen on
    #[clap(long, default_value_t = 3000)]
    pub port: u64,

    /// Force UNLINK before DELETE
    #[clap(long, num_args = 0, default_value_t = false)]
    pub protect: bool,

    /// Amount of replicas to make of the data
    #[clap(long, value_name = "AMOUNT", default_value_t = 3)]
    pub replicas: u64,

    /// Volumes to use for storage, comma separated
    #[clap(long = "volumes", value_name = "VOLUMES", required = true)]
    pub pvolumes: Option<String>,

    /// Amount of subvolumes, disk per machine
    #[clap(long, value_name = "AMOUNT", default_value_t = 10)]
    pub subvolumes: u64,

    /// Volume servers must respond to GET/HEAD requests in this amount of
    /// time or they are considered down, as duration
    #[clap(long, hide = true, default_value_t = 1)]
    pub voltimeout: u64,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Server,
    /// Change the amount of volume servers
    Rebuild,
    /// Regenerate LevelDB
    Rebalance,
}

impl Options {
    pub fn run() -> Result<()> {
        dbg!(Self::parse());
        Commands::run(Self::parse().command)
    }

    pub fn turner() -> Self {
        Self::parse()
    }

    pub fn get_string(value: Option<String>) -> Result<String> {
        if let Some(v) = value {
            return Ok(v);
        }

        Ok("".to_string())
    }
}

impl Commands {
    pub fn run(self) -> Result<()> {
        match self {
            Commands::Server => server::run(),
            Commands::Rebuild => rebuild::run(),
            Commands::Rebalance => rebalance::run(),
        }
    }
}
