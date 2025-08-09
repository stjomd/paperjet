use colored::Colorize;
use std::process;

mod cli;

fn main() {
	let args = cli::args::Args::parse();
	if let Err(err) = cli::run_command(args) {
		eprintln!("{} {}", "error:".bold().red(), err);
		process::exit(1);
	}
}
