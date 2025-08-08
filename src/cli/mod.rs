use anyhow::Result;

use crate::cli::args::{Args, Command};

pub mod args;
pub mod commands;
mod common;
mod pdf;
mod snapshot;

/// Runs the command specified in the [`Args`] instance.
pub fn run_command(args: Args) -> Result<()> {
	match args.command {
		Command::List => commands::list(),
		Command::Display(d_args) => commands::display(d_args),
		Command::Print(p_args) => commands::print(p_args),
	}
}
