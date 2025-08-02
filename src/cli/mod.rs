use crate::cli::args::{Args, Command};
use crate::cli::error::CliError;

pub mod args;
pub mod commands;
mod common;
pub mod error;
mod snapshot;

/// Runs the command specified in the [`Args`] instance.
pub fn run_command(args: Args) -> Result<(), CliError> {
	match args.command {
		Command::List => commands::list(),
		Command::Display(d_args) => commands::display(d_args),
		Command::Print(p_args) => commands::print(p_args),
	}
}
