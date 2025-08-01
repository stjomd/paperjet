use crate::cli::args::{Args, Command};

pub mod args;
mod common;
mod display;
mod list;
mod print;

/// Runs the command specified in the [`Args`] instance.
pub fn run_command(args: Args) {
	match args.command {
		Command::List => list::list(),
		Command::Display(d_args) => display::display(d_args),
		Command::Print(p_args) => print::print(p_args).expect("TODO"),
	}
}
