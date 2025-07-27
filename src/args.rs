use clap::{ArgAction, Parser, Subcommand};

#[derive(Parser)]
#[command(version, disable_version_flag = true)]
pub struct Args {
	#[command(subcommand)]
	pub command: Command,
	/// Print version
	#[arg(long, action = ArgAction::Version)]
	pub version: Option<bool>,
}

#[derive(Subcommand)]
#[command(disable_help_subcommand = true)]
pub enum Command {
	/// Lists available printers.
	List(ListArgs),
}

#[derive(clap::Args)]
pub struct ListArgs {
	#[arg(short, long)]
	/// List more information about each printer.
	detailed: bool,
}

pub fn parse() -> Args {
	Args::parse()
}
