use clap::{ArgAction, Parser, Subcommand};
use printrs::options::{
	ColorMode, Finishing, MediaSize, MediaSource, MediaType, Orientation, Quality, SidesMode,
};
use std::path;

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
	/// Submits one or more documents for printing.
	///
	/// This command supports extensive configuration of options such as the amount of copies,
	/// paper size, orientation, and others, listed below.
	/// Support is up to a particular device - unsupported options or option values will be rejected
	/// and the printing will not commence.
	Print(PrintArgs),
}

#[derive(clap::Args)]
pub struct DisplayArgs {
	/// The ID of the printer (as determined by the `list` command).
	pub id: usize,
	/// Display all options of the printer.
	#[arg(short, long)]
	pub options: bool,
}

#[derive(Debug, clap::Args)]
pub struct PrintArgs {
	/// Paths to the files to be printed.
	///
	/// File extensions, types, or contents are not validated.
	/// Support will be determined by the device's driver.
	#[arg(value_name = "files", required = true, num_args = 1..)]
	pub paths: Vec<path::PathBuf>,

	/// Amount of copies [default: 1]
	///
	/// In case of multiple files, this option applies to each of them.
	#[arg(short, long)]
	pub copies: Option<usize>,

	/// Finishing processes to be performed by the printer.
	#[arg(short, long, value_delimiter = ',')]
	pub finishings: Option<Vec<Finishing>>,

	/// Size of the media, most often paper size.
	#[arg(short, long)]
	pub size: Option<MediaSize>,

	/// Source where the media is pulled from.
	#[arg(short = 'r', long)]
	pub source: Option<MediaSource>,

	/// Type of media.
	#[arg(short = 't', long)]
	pub media_type: Option<MediaType>,

	/// Number of document pages per media side [default: 1]
	#[arg(short = 'u', long)]
	pub number_up: Option<usize>,

	/// Orientation of document pages.
	#[arg(short, long)]
	pub orientation: Option<Orientation>,

	/// Determines whether the printer should use color or monochrome ink.
	#[arg(short = 'm', long)]
	pub color_mode: Option<ColorMode>,

	/// The quality of the resulting print.
	#[arg(short, long)]
	pub quality: Option<Quality>,

	/// Determines whether only one or both sides of the media should be printed on.
	#[arg(short = 'd', long)]
	pub sides_mode: Option<SidesMode>,
}

impl Args {
	pub fn parse() -> Self {
		<Self as Parser>::parse()
	}
}
