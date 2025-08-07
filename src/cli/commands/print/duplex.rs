use std::io::{self, Cursor, Read, Seek, Write};

use anyhow::{Result, bail};
use colored::Colorize;
use printrs::Printer;
use printrs::options::PrintOptions;

use crate::cli::pdf;

pub fn begin_printing<I, R>(readers: I, printer: Printer, options: PrintOptions) -> Result<()>
where
	I: IntoIterator<Item = R>,
	R: Read + Seek,
{
	let mut readers = readers.into_iter().collect::<Vec<_>>();
	// Validate amount of files
	if readers.len() != 1 {
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

	let pdfium = pdf::pdfium()?;
	let file = readers.remove(0);

	let (front_pdf, back_pdf) = pdf::split::split_pdf(&pdfium, file)?;
	let (front_len, back_len) = (front_pdf.pages().len(), back_pdf.pages().len());
	let (front_bytes, back_bytes) = pdf::split::pdfs_to_bytes(front_pdf, back_pdf)?;

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
