use crate::cli::args::Args;

mod cli;

fn main() {
	let args = Args::parse();
	cli::run_command(args);
}
