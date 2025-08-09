use std::io::{self, Cursor, Read, Seek, Write};

use anyhow::{Result, anyhow, bail};
use colored::Colorize;
use paperjet::Printer;
use paperjet::options::PrintOptions;
use pdfium_render::prelude::*;

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
	let (front, back) = extract_first_two(readers)
		.ok_or_else(|| anyhow!("could not access two readers for the respective sides"))?;

	let options = PrintOptions::from(args);
	validate_options(&options)?;

	// Determine the amount of sheets required:
	// `front` and `back` must have the same amount of pages (precondition), thus it's enough
	// to just load one of the two.
	let (front, sheets_num) = get_number_of_pages(front)?;
	let front = Cursor::new(front);
	println!(
		"You will need {} {} of paper.",
		sheets_num.to_string().bold().cyan(),
		if sheets_num == 1 { "sheet" } else { "sheets" },
	);

	// Start interactions
	println!("\nPrinting the front side...");
	paperjet::print([front], printer.clone(), options.clone())?;
	println!("The front side has been submitted.");

	print!(
		"\nOnce the printing has finished, turn the pages over and press {}: ",
		"Enter".bold().cyan()
	);
	io::stdout().flush()?;
	let mut input = String::new();
	io::stdin().read_line(&mut input)?;

	println!("\nPrinting the back side...");
	paperjet::print([back], printer, options)?;
	println!("The back side has been submitted.");
	Ok(())
}

/// Consumes an iterable collection and returns its first two elements as an owned value.
/// If the collection has less than two elements, returns `None`.
fn extract_first_two<I, T>(collection: I) -> Option<(T, T)>
where
	I: IntoIterator<Item = T>,
{
	let mut iter = collection.into_iter();
	match (iter.next(), iter.next()) {
		(Some(first), Some(second)) => Some((first, second)),
		_ => None,
	}
}

/// Validates `options` for compatibility with duplex printing mode.
/// Returns `Ok` if the validation passed, and `Err` otherwise.
fn validate_options(options: &PrintOptions) -> Result<()> {
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
	Ok(())
}

/// Consumes a reader of a PDF document and returns a tuple containing its contents, serialized
/// to bytes, as an owned value, and the amount of pages.
fn get_number_of_pages<R>(reader: R) -> Result<(Vec<u8>, PdfPageIndex)>
where
	R: Read + Seek,
{
	let pdfium = pdf::pdfium()?;
	let front_pdf = pdfium.load_pdf_from_reader(reader, None)?;
	let sheets_num = front_pdf.pages().len();
	Ok((front_pdf.save_to_bytes()?, sheets_num))
}

#[cfg(test)]
mod tests {
	use paperjet::options::{CopiesInt, NumberUpInt, SidesMode};

	use super::*;

	#[test]
	fn if_copies_is_set_then_options_invalid() {
		// Set options with a defined copies parameter
		let options = PrintOptions {
			copies: Some(CopiesInt(1)),
			..Default::default()
		};

		// Validation should fail
		assert!(validate_options(&options).is_err());
	}

	#[test]
	fn if_number_up_is_set_then_options_invalid() {
		// Set options with a defined copies parameter
		let options = PrintOptions {
			number_up: Some(NumberUpInt(1)),
			..Default::default()
		};

		// Validation should fail
		assert!(validate_options(&options).is_err());
	}

	#[test]
	fn if_sides_mode_is_set_then_options_invalid() {
		// Set options with a defined copies parameter
		let options = PrintOptions {
			sides_mode: Some(SidesMode::TwoSidedLandscape),
			..Default::default()
		};

		// Validation should fail
		assert!(validate_options(&options).is_err());
	}
}
