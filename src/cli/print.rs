use std::fs::File;

use printrs::error::PrintError;
use printrs::options::PrintOptions;

use crate::cli::args::PrintArgs;

/// The `print` command
pub fn print(args: PrintArgs) -> Result<(), PrintError> {
	let files: Vec<File> = args
		.paths
		.iter()
		.map(File::open)
		.collect::<Result<_, _>>()?;

	let options = PrintOptions::from(args);
	printrs::print(files, options).inspect(|_| println!("Files have been submitted for printing."))
}
