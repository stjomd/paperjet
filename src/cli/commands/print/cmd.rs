use std::fs::File;
use std::io::Cursor;
use std::path::PathBuf;

use anyhow::{Result, anyhow};
use colored::Colorize;
use printrs::Printer;
use printrs::options::PrintOptions;

use crate::cli::args::PrintArgs;
use crate::cli::commands::print::{duplex, transform};
use crate::cli::common;

// TODO: remove io::Seek requirement on printrs::print (required by duplex::print <- split_pdf)

/// The `print` command
pub fn print(args: PrintArgs) -> Result<()> {
	let files = open_files(&args.paths)?;
	let printer = select_printer(&args)?;

	// Transform
	let documents = transform::transform(files, &args)?
		.into_iter()
		.map(Cursor::new)
		.collect::<Vec<_>>();

	// Duplex mode is interactive and will submit to print on its own: hence the return
	if args.duplex {
		return duplex::begin_printing(documents, printer, &args);
	}

	// Simplex mode: submit to print
	let options = PrintOptions::from(&args);
	printrs::print(documents, printer, options)?;
	println!("Files have been submitted for printing.");

	Ok(())
}

/// Converts a collection of paths into a collection of files at those paths.
/// Returns `Ok` if all files could be opened, or `Err` if at least one file could not be opened
/// (the error refers to the first file that could not be opened).
fn open_files(paths: &[PathBuf]) -> Result<Vec<File>> {
	paths
		.iter()
		.map(|path| {
			File::open(path).map_err(|e| {
				anyhow!(
					"could not open file '{}': {}",
					path.display().to_string().yellow(),
					e,
				)
			})
		})
		.collect::<Result<_>>()
}

/// Selects a printer according to the arguments.
fn select_printer(args: &PrintArgs) -> Result<Printer> {
	if let Some(id) = args.printer_id {
		common::get_printer_by_id(id).ok_or(anyhow!(
			"could not find a printer by the ID: '{}'",
			id.to_string().yellow()
		))
	} else if let Some(ref name) = args.printer_name {
		common::get_printer_by_name(name).ok_or(anyhow!(
			"could not find a printer by the name: '{}'",
			name.yellow()
		))
	} else {
		printrs::get_default_printer().ok_or(anyhow!("no default printer is available"))
	}
}
