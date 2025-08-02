use clap::builder::Styles;
use clap::builder::styling::AnsiColor;
use clap::{ArgAction, Parser, Subcommand};
use printrs::options::{
	ColorMode, Finishing, MediaSize, MediaSource, MediaType, Orientation, Quality, SidesMode,
};
use std::ffi::c_int;
use std::path::PathBuf;

mod headings {
	pub const MISC: &str = "Miscellaneous";
}

#[derive(Parser)]
#[command(version, disable_version_flag = true, disable_help_flag = true, styles = help_style())]
pub struct Args {
	#[command(subcommand)]
	pub command: Command,

	/// Output more information to the console.
	#[arg(short, long, global = true, help_heading = headings::MISC)]
	pub verbose: bool,

	/// Print version.
	#[arg(long, action = ArgAction::Version, help_heading = headings::MISC)]
	pub version: Option<bool>,

	/// Print help.
	#[arg(short, long, global = true, action = ArgAction::Help, help_heading = headings::MISC)]
	pub help: Option<bool>,
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
	pub paths: Vec<PathBuf>,

	/// Amount of copies [default: 1]
	///
	/// In case of multiple files, this option applies to each of them.
	#[arg(short, long, value_parser = clap::value_parser!(c_int).range(1..))]
	pub copies: Option<c_int>,

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
	#[arg(short = 'u', long, value_parser = clap::value_parser!(c_int).range(1..))]
	pub number_up: Option<c_int>,

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

fn help_style() -> Styles {
	Styles::styled().placeholder(AnsiColor::White.on_default().dimmed())
}
