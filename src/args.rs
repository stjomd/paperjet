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
	List,
	/// Displays information about a printer.
	Display(DisplayArgs),
}

#[derive(clap::Args)]
pub struct DisplayArgs {
	/// The ID of the printer (as determined by the `list` command).
	pub id: usize,
	/// Display all options of the printer.
	#[arg(short, long)]
	pub options: bool,
}

impl Args {
	pub fn parse() -> Self {
		<Self as Parser>::parse()
	}
}
