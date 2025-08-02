use colored::Colorize;

use crate::cli::args::{Args, Command};

pub mod args;
mod common;
mod display;
mod list;
mod print;
mod snapshot;

/// Runs the command specified in the [`Args`] instance.
pub fn run_command(args: Args) {
	match args.command {
		Command::List => list::list(),
		Command::Display(d_args) => display::display(d_args),
		Command::Print(p_args) => {
			let result = print::print(p_args);
			if let Err(err) = result {
				eprintln!("{} {}", "error:".bold().red(), err)
			}
		}
	}
}
