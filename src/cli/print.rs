use std::fs::File;

use printrs::error::PrintError;

use crate::cli::args::PrintArgs;

/// The `print` command
pub fn print(args: PrintArgs) -> Result<(), PrintError> {
	let files: Vec<File> = args
		.paths
		.iter()
		.map(File::open)
		.collect::<Result<_, _>>()?;

	printrs::print(files).inspect(|_| println!("Files have been submitted for printing."))
}
