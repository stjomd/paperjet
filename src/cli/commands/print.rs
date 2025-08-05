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
	// Validate amount of files
	if files.len() != 1 {
		bail!("exactly one file must be specified to print in duplex mode")
	}
	// Validate options
	if options.copies.is_some() {
		bail!(
			"option '{}' is not supported in duplex mode",
			"copies".yellow()
		)
	} else if options.number_up.is_some() {
		bail!(
			"option '{}' is not supported in duplex mode",
			"number up".yellow()
		)
	} else if options.sides_mode.is_some() {
		bail!(
			"option '{}' is not supported in duplex mode",
			"sides mode".yellow()
		)
	}

	let pdfium = pdfium()?;
	let file = files.remove(0);

	let (front_pdf, back_pdf) = split_pdf(&pdfium, file)?;
	let (front_len, back_len) = (front_pdf.pages().len(), back_pdf.pages().len());
	let (front_bytes, back_bytes) = pdfs_to_bytes(front_pdf, back_pdf)?;

	let sheets_num = u16::max(front_len, back_len);
	println!(
		"You will need {} {} of paper.",
		sheets_num.to_string().bold().cyan(),
		if sheets_num == 1 { "sheet" } else { "sheets" },
	);

	println!("\nPrinting the front side...");
	printrs::print([Cursor::new(front_bytes)], printer.clone(), options.clone())?;
	println!("The front side has been submitted.");

	print!(
		"\nOnce the printing has finished, turn the pages over and press {}: ",
		"Enter".bold().cyan()
	);
	io::stdout().flush()?;
	let mut input = String::new();
	io::stdin().read_line(&mut input)?;

	println!("\nPrinting the back side...");
	printrs::print([Cursor::new(back_bytes)], printer, options)?;
	println!("The back side has been submitted.");
	Ok(())
}

/// Loads PDFium as a dynamic library.
fn pdfium() -> Result<Pdfium> {
	Ok(Pdfium::new(
		Pdfium::bind_to_library(Pdfium::pdfium_platform_library_name_at_path("./"))
			.or_else(|_| Pdfium::bind_to_system_library())
			.map_err(|_e| match _e {
				PdfiumError::LoadLibraryError(e) => anyhow!("could not link PDFium: {e}"),
				e => anyhow!("could not link PDFium: {e}"),
			})?,
	))
}

/// Splits a provided `pdf_file` into two PDF documents: for the front and back side.
fn split_pdf<'a>(pdfium: &'a Pdfium, pdf_file: File) -> Result<(PdfDocument<'a>, PdfDocument<'a>)> {
	let source = pdfium.load_pdf_from_reader(pdf_file, None)?;
	let mut front = pdfium.create_new_pdf()?;
	let mut back = pdfium.create_new_pdf()?;

	// Alternate between copying pages to `front` and `back`.
	// When printing the back side, the printed front sides will be flipped, thus we need to copy
	// the back side pages in reverse order.
	for i in 0..source.pages().len() {
		if i % 2 == 0 {
			let j = front.pages().len();
			front.pages_mut().copy_page_from_document(&source, i, j)?;
		} else {
			back.pages_mut().copy_page_from_document(&source, i, 0)?;
		};
	}

	// If the total amount of pages is uneven, we need to insert one blank page at the beginning
	// of the back side - this aligns the two prints.
	let page_size = source.pages().page_size(0)?;
	align_sides(&mut front, &mut back, page_size)?;

	Ok((front, back))
}

/// Aligns two split PDF documents, `front` and `back`, by inserting a single blank page
/// at the beginning of the back side document.
fn align_sides(front: &mut PdfDocument, back: &mut PdfDocument, dimensions: PdfRect) -> Result<()> {
	if back.pages().len() != front.pages().len() {
		back.pages_mut()
			.create_page_at_start(PdfPagePaperSize::Custom(
				dimensions.width(),
				dimensions.height(),
			))?;
	}
	Ok(())
}

/// Converts two split PDF documents, `even` and `odd`, to bytes.
fn pdfs_to_bytes<'a>(even: PdfDocument<'a>, odd: PdfDocument<'a>) -> Result<(Vec<u8>, Vec<u8>)> {
	Ok((even.save_to_bytes()?, odd.save_to_bytes()?))
}
