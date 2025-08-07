use std::io::{self, Cursor, Read, Seek, Write};

use anyhow::{Result, bail};
use colored::Colorize;
use printrs::Printer;
use printrs::options::PrintOptions;

use crate::cli::args::PrintArgs;
use crate::cli::pdf;

/// Starts the interactive duplex printing mode.
///
/// `readers` must contain at least two documents, for each side (front and back).
/// The amount of pages must be the same between the two.
/// If `readers` contains more than 2 documents, the documents after the second are ignored.
pub fn begin_printing<I, R>(readers: I, printer: Printer, args: &PrintArgs) -> Result<()>
where
	I: IntoIterator<Item = R>,
	R: Read + Seek,
{
	let (front, back) = {
		let mut iter = readers.into_iter();
		match (iter.next(), iter.next()) {
			(Some(first), Some(second)) => (first, second),
			_ => bail!("could not access two readers for the respective sides"),
		}
	};

	// Validate options
	let options = PrintOptions::from(args);
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

	// Determine the amount of sheets required:
	// `front` and `back` must have the same amount of pages (precondition), thus it's enough
	// to just load one of the two.
	let pdfium = pdf::pdfium()?;
	let front_pdf = pdfium.load_pdf_from_reader(front, None)?;
	let sheets_num = front_pdf.pages().len();
	let front = Cursor::new(front_pdf.save_to_bytes()?);
	println!(
		"You will need {} {} of paper.",
		sheets_num.to_string().bold().cyan(),
		if sheets_num == 1 { "sheet" } else { "sheets" },
	);

	// Start interactions
	println!("\nPrinting the front side...");
	printrs::print([front], printer.clone(), options.clone())?;
	println!("The front side has been submitted.");

	print!(
		"\nOnce the printing has finished, turn the pages over and press {}: ",
		"Enter".bold().cyan()
	);
	io::stdout().flush()?;
	let mut input = String::new();
	io::stdin().read_line(&mut input)?;

	println!("\nPrinting the back side...");
	printrs::print([back], printer, options)?;
	println!("The back side has been submitted.");
	Ok(())
}
