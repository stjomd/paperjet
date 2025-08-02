use std::fs::File;

use printrs::error::PrintError;
use printrs::options::PrintOptions;

use crate::cli::args::PrintArgs;
use crate::cli::common;

/// The `print` command
pub fn print(args: PrintArgs) -> Result<(), PrintError> {
	let files: Vec<File> = args
		.paths
		.iter()
		.map(File::open)
		.collect::<Result<_, _>>()?;

	let printer = match args.printer_id {
		Some(id) => common::get_printer_by_id(id).ok_or(PrintError::NoPrinters),
		None => printrs::get_default_printer().ok_or(PrintError::NoPrinters),
	}?;

	let options = PrintOptions::from(args);
	printrs::print(files, printer, options)
		.inspect(|_| println!("Files have been submitted for printing."))
}
