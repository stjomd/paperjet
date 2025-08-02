use colored::Colorize;

use crate::cli::args::{Args, Command};

pub mod args;
pub mod commands;
mod common;
mod snapshot;

/// Runs the command specified in the [`Args`] instance.
pub fn run_command(args: Args) {
	match args.command {
		Command::List => commands::list(),
		Command::Display(d_args) => commands::display(d_args),
		Command::Print(p_args) => {
			let result = commands::print(p_args);
			if let Err(err) = result {
				eprintln!("{} {}", "error:".bold().red(), err)
			}
		}
	}
}
