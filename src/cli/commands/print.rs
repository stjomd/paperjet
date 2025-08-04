use std::fs::File;
use std::io::{self, Cursor, Write};
use std::path::PathBuf;

use anyhow::{Result, anyhow, bail};
use colored::Colorize;
use pdfium_render::prelude::*;
use printrs::Printer;
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

	let is_duplex_mode = args.duplex;
	let options = PrintOptions::from(args);

	if is_duplex_mode {
		duplex(files, printer, options)
	} else {
		printrs::print(files, printer, options)
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

fn duplex(mut files: Vec<File>, printer: Printer, options: PrintOptions) -> Result<()> {
	if files.len() != 1 {
		bail!("exactly one file must be specified to print in duplex mode")
	}
	let file = files.remove(0);
	let split = split_pdf(file)?;

	println!("Printing one side...");
	printrs::print([Cursor::new(split.0)], printer.clone(), options.clone())?;
	println!("One side has been submitted.");

	print!("Once the printing has finished, turn the pages over and press Enter: ");
	io::stdout().flush()?;
	let mut input = String::new();
	io::stdin().read_line(&mut input)?;

	println!("Printing the other side...");
	printrs::print([Cursor::new(split.1)], printer, options)?;
	println!("The other side has been submitted.");
	Ok(())
}

fn split_pdf(pdf_file: File) -> Result<SplitPdf> {
	let pdfium = Pdfium::new(
		Pdfium::bind_to_library(Pdfium::pdfium_platform_library_name_at_path("./"))
			.or_else(|_| Pdfium::bind_to_system_library())
			.map_err(|e| match e {
				PdfiumError::LoadLibraryError(_e) => anyhow!("could not link PDFium: {_e}"),
				_e => anyhow!("could not link PDFium: {_e}"),
			})?,
	);

	let source = pdfium.load_pdf_from_reader(pdf_file, None)?;
	let mut even = pdfium.create_new_pdf()?;
	let mut odd = pdfium.create_new_pdf()?;

	for i in 0..source.pages().len() {
		if i % 2 == 0 {
			let j = even.pages().len();
			even.pages_mut().copy_page_from_document(&source, i, j)?;
		} else {
			odd.pages_mut().copy_page_from_document(&source, i, 0)?;
		};
	}
	if odd.pages().len() < even.pages().len() {
		let page_size = source.pages().page_size(0)?;
		odd.pages_mut()
			.create_page_at_start(PdfPagePaperSize::Custom(
				page_size.width(),
				page_size.height(),
			))?;
	}

	let split = SplitPdf(
		even.save_to_bytes().map_err(anyhow::Error::from)?,
		odd.save_to_bytes().map_err(anyhow::Error::from)?,
	);
	Ok(split)
}

struct SplitPdf(Vec<u8>, Vec<u8>);
