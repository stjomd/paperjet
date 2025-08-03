use std::fs::File;
use std::path::PathBuf;

use printrs::error::PrintError;
use printrs::options::PrintOptions;

use crate::cli::args::PrintArgs;
use crate::cli::common;
use crate::cli::error::CliError;

/// The `print` command
pub fn print(args: PrintArgs) -> Result<(), CliError> {
	let files: Vec<File> = args
		.paths
		.iter()
		.map(map_path_to_file_result)
		.collect::<Result<_, _>>()?;

	let printer = if let Some(id) = args.printer_id {
		common::get_printer_by_id(id).ok_or(CliError::PrinterNotFoundById(id))?
	} else if let Some(ref name) = args.printer_name {
		common::get_printer_by_name(name)
			.ok_or_else(|| CliError::PrinterNotFoundByName(name.clone()))?
	} else {
		printrs::get_default_printer().ok_or(PrintError::NoPrinters)?
	};

	let options = PrintOptions::from(args);
	printrs::print(files, printer, options)
		.inspect(|_| println!("Files have been submitted for printing."))?;
	Ok(())
}

fn map_path_to_file_result(path: &PathBuf) -> Result<File, CliError> {
	File::open(path).map_err(|e| CliError::FileError {
		path: path.clone(),
		source: e,
	})
}
