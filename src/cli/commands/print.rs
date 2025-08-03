use std::fs::File;
use std::path::PathBuf;

use anyhow::{Result, anyhow};
use colored::Colorize;
use printrs::error::PrintError;
use printrs::options::PrintOptions;

use crate::cli::args::PrintArgs;
use crate::cli::common;

/// The `print` command
pub fn print(args: PrintArgs) -> Result<()> {
	let files: Vec<File> = args
		.paths
		.iter()
		.map(map_path_to_file_result)
		.collect::<Result<_>>()?;

	let printer = if let Some(id) = args.printer_id {
		common::get_printer_by_id(id).ok_or(anyhow!(
			"could not find a printer by the ID: '{}'",
			id.to_string().yellow()
		))?
	} else if let Some(ref name) = args.printer_name {
		common::get_printer_by_name(name)
			.ok_or_else(|| anyhow!("could not find a printer by the name: '{}'", name.yellow()))?
	} else {
		printrs::get_default_printer().ok_or(PrintError::NoPrinters)?
	};

	let options = PrintOptions::from(args);
	printrs::print(files, printer, options)
		.inspect(|_| println!("Files have been submitted for printing."))?;
	Ok(())
}

/// Opens the file at the specified path and returns it.
fn map_path_to_file_result(path: &PathBuf) -> Result<File> {
	File::open(path).map_err(|e| {
		anyhow!(
			"could not open file '{}': {}",
			path.display().to_string().yellow(),
			e
		)
	})
}
