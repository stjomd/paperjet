use std::ops::RangeInclusive;

use anyhow::{Result, anyhow, bail};
use colored::Colorize;
use pdfium_render::prelude::*;

/// Loads PDFium as a dynamic library.
pub fn pdfium() -> Result<Pdfium> {
	Ok(Pdfium::new(
		Pdfium::bind_to_library(Pdfium::pdfium_platform_library_name_at_path("./"))
			.or_else(|_| Pdfium::bind_to_system_library())
			.map_err(|_e| match _e {
				PdfiumError::LoadLibraryError(e) => anyhow!("could not link PDFium: {e}"),
				e => anyhow!("could not link PDFium: {e}"),
			})?,
	))
}

pub fn slice_document<'a>(
	pdfium: &'a Pdfium,
	source: &PdfDocument<'a>,
	range: RangeInclusive<PdfPageIndex>,
) -> Result<PdfDocument<'a>> {
	if source.pages().is_empty() {
		bail!("document is empty");
	}

	// Validate range (from user point of view, indexing starts with 1)
	let pages_len = source.pages().len();
	if *range.start() < 1 {
		bail!(
			"range must start from {}: provided {}",
			"1".green(),
			range.start().to_string().yellow(),
		)
	}
	if *range.start() > pages_len {
		bail!(
			"document has {} pages, but range is starting from {}",
			pages_len.to_string().green(),
			range.start().to_string().yellow(),
		)
	}
	if *range.end() > pages_len {
		bail!(
			"document has {} pages, but range is ending at {}",
			pages_len.to_string().green(),
			range.end().to_string().yellow(),
		)
	}
	if *range.start() > *range.end() {
		bail!(
			"range is empty: from {} to {}",
			range.start().to_string().yellow(),
			range.end().to_string().yellow(),
		)
	}

	let idx_range = (range.start() - 1)..=(range.end() - 1);
	let mut new = pdfium.create_new_pdf()?;
	new.pages_mut()
		.copy_page_range_from_document(source, idx_range, 0)?;
	Ok(new)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	#[allow(clippy::reversed_empty_ranges)]
	fn if_empty_source_then_err() {
		let pdfium = pdfium().expect("PDFium should be available, but isn't");

		// Create an empty source document
		let source = pdfium.create_new_pdf().expect("Could not create a new PDF");

		// No matter what the passed in range is, result should be Err
		assert!(
			slice_document(&pdfium, &source, 0..=0).is_err(),
			"slice_document should return Err, but returned Ok"
		);
		assert!(
			slice_document(&pdfium, &source, 0..=5).is_err(),
			"slice_document should return Err, but returned Ok"
		);
		assert!(
			slice_document(&pdfium, &source, 5..=0).is_err(),
			"slice_document should return Err, but returned Ok"
		);
	}

	#[test]
	fn if_source_has_pages_and_valid_range_then_ok() {
		let pdfium = pdfium().expect("PDFium should be available, but isn't");

		// Create a source document
		let pages_len = 5;
		let mut source = pdfium.create_new_pdf().expect("Could not create a new PDF");
		for _ in 0..pages_len {
			source
				.pages_mut()
				.create_page_at_start(PdfPagePaperSize::a4())
				.expect("Could not create page");
		}

		// As long as the range is valid, result should be Ok, and contain appropriate number of pages
		let sliced = slice_document(&pdfium, &source, 2..=3).expect("Could not slice document");
		assert_eq!(
			2,
			sliced.pages().len(),
			"sliced document must have {} pages, has: {}",
			2,
			sliced.pages().len()
		);

		let sliced = slice_document(&pdfium, &source, 1..=5).expect("Could not slice document");
		assert_eq!(
			5,
			sliced.pages().len(),
			"sliced document must have {} pages, has: {}",
			5,
			sliced.pages().len()
		);
	}

	#[test]
	fn if_source_has_pages_and_invalid_range_then_err() {
		let pdfium = pdfium().expect("PDFium should be available, but isn't");

		// Create a source document
		let pages_len = 5;
		let mut source = pdfium.create_new_pdf().expect("Could not create a new PDF");
		for _ in 0..pages_len {
			source
				.pages_mut()
				.create_page_at_start(PdfPagePaperSize::a4())
				.expect("Could not create page");
		}

		// As long as the range is invalid, results should be Err:
		let result = slice_document(&pdfium, &source, 0..=3);
		assert!(
			result.is_err(),
			"range starting with 0 is invalid, but Ok was returned"
		);

		let result = slice_document(&pdfium, &source, 1..=6);
		assert!(
			result.is_err(),
			"range ending with 6 is invalid (as document has {pages_len} pages), but Ok was returned",
		);

		#[allow(clippy::reversed_empty_ranges)]
		let result = slice_document(&pdfium, &source, 4..=2);
		assert!(result.is_err(), "range is empty, but Ok was returned");
	}

	#[test]
	fn if_source_has_exactly_one_page_and_some_range_then_returns_correct_result() {
		let pdfium = pdfium().expect("PDFium should be available, but isn't");

		// Create a source document with one page
		let mut source = pdfium.create_new_pdf().expect("Could not create a new PDF");
		source
			.pages_mut()
			.create_page_at_start(PdfPagePaperSize::a4())
			.expect("Could not create page");

		// The only valid range for a document with one page is 1..=1:
		let result = slice_document(&pdfium, &source, 1..=1).expect("valid range must return Ok");
		assert_eq!(
			1,
			result.pages().len(),
			"sliced document must have {} page, but has: {}",
			1,
			result.pages().len()
		);

		// Any other range is invalid:
		let result = slice_document(&pdfium, &source, 0..=1);
		assert!(result.is_err(), "range is invalid, but Ok was returned");

		let result = slice_document(&pdfium, &source, 1..=2);
		assert!(result.is_err(), "range is invalid, but Ok was returned");

		let result = slice_document(&pdfium, &source, 0..=2);
		assert!(result.is_err(), "range is invalid, but Ok was returned");

		#[allow(clippy::reversed_empty_ranges)]
		let result = slice_document(&pdfium, &source, 2..=1);
		assert!(result.is_err(), "range is invalid, but Ok was returned");
	}
}
