use anyhow::{Context, Result};
use clap::Parser;


#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
	#[arg(long, short)]
	pub dry_run: bool,
	#[arg(required = true)]
	pub flow: String,
	pub origin: Option<String>,
}

pub fn parse() -> Result<Cli> {
	Cli::try_parse().context("Failed to parse CLI arguments")
}
