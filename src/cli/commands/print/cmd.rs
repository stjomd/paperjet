use std::fs::File;
use std::io::{Cursor, Read, Seek, Write};
use std::path::PathBuf;

use anyhow::{Result, anyhow, bail};
use colored::Colorize;
use printrs::Printer;
use printrs::error::PrintError;
use printrs::options::PrintOptions;

use crate::cli::args::PrintArgs;
use crate::cli::commands::print::{duplex, pdf};
use crate::cli::common;

// TODO: refactor this mess
// TODO: remove io::Seek requirement on printrs::print (required by duplex::print <- split_pdf)

// 1. args.from/to.is_some()
//		  yes -> slice and get bytes
//		  no -> keep files
// 2. args.duplex == true
//			yes -> split and get two bytes vecs (io::Read) => interactive printing
//			no -> keep files => normal printing

/// The `print` command
pub fn print(args: PrintArgs) -> Result<()> {
	let files: Vec<File> = args
		.paths
		.iter()
		.map(map_path_to_file_result)
		.collect::<Result<_>>()?;

	// Printer selection
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

	// Printing + slicing
	if args.from.is_some() || args.to.is_some() {
		// Slice document, then print that
		let sliced_document = slice_document(files, &args)?;
		let mut file = File::create("test.pdf")?;
		file.write_all(&sliced_document.into_inner())?;
		Ok(())
		// start_printing([sliced_document], printer, args)
	} else {
		// Just start printing
		start_printing(files, printer, args)
	}
}

fn slice_document(mut files: Vec<File>, args: &PrintArgs) -> Result<Cursor<Vec<u8>>> {
	if files.len() != 1 {
		bail!("exactly one file must be specified to slice a document")
	}
	let file = files.remove(0);
	let pdfium = pdf::pdfium()?;

	let source = pdfium.load_pdf_from_reader(file, None)?;
	let sliced = pdf::slice_document(&pdfium, &source, args.from, args.to)?;

	let bytes = sliced.save_to_bytes()?;
	Ok(Cursor::new(bytes))
}

fn start_printing<I, R>(readers: I, printer: Printer, args: PrintArgs) -> Result<()>
where
	I: IntoIterator<Item = R>,
	R: Read + Seek,
{
	let is_duplex = args.duplex;
	let options = PrintOptions::from(args);
	if is_duplex {
		duplex::print(readers, printer, options)
	} else {
		printrs::print(readers, printer, options)
			.inspect(|_| println!("Files have been submitted for printing."))
			.map_err(anyhow::Error::from)
	}
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
