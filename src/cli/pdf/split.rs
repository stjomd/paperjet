use anyhow::{Result, bail};
use colored::Colorize;
use pdfium_render::prelude::*;

/// Splits a provided `source` PDF file into two PDF documents: for the front and back side.
pub fn split_pdf<'a>(
	pdfium: &'a Pdfium,
	source: &PdfDocument<'a>,
) -> Result<(PdfDocument<'a>, PdfDocument<'a>)> {
	let pages_len = source.pages().len();
	if pages_len < 2 {
		bail!(
			"document only has {} {}: must have at least {} pages to print in duplex mode",
			pages_len.to_string().yellow(),
			if pages_len == 1 { "page" } else { "pages" },
			"2".green()
		)
	}

	let mut front = pdfium.create_new_pdf()?;
	let mut back = pdfium.create_new_pdf()?;
	// Alternate between copying pages to `front` and `back`.
	// When printing the back side, the printed front sides will be flipped, thus we need to copy
	// the back side pages in reverse order.
	for i in 0..pages_len {
		if i % 2 == 0 {
			let j = front.pages().len();
			front.pages_mut().copy_page_from_document(source, i, j)?;
		} else {
			back.pages_mut().copy_page_from_document(source, i, 0)?;
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

#[cfg(test)]
mod tests {
	use super::*;
	use crate::cli::pdf;

	#[test]
	fn if_empty_pdf_then_split_returns_err() {
		let pdfium = pdf::pdfium().expect("PDFium should be available, but isn't");

		// Create an empty PDF file:
		let pdf = pdfium
			.create_new_pdf()
			.expect("Could not create new PDF with PDFium");

		// Attempt to split the file: it has 0 pages, and thus fails
		let result = split_pdf(&pdfium, &pdf);
		assert!(
			result.is_err(),
			"split_pdf should return an error, but didn't"
		);
	}

	#[test]
	fn if_pdf_with_one_page_then_split_returns_err() {
		let pdfium = pdf::pdfium().expect("PDFium should be available, but isn't");

		// Create a PDF file with one page:
		let mut pdf = pdfium
			.create_new_pdf()
			.expect("Could not create new PDF with PDFium");
		pdf.pages_mut()
			.create_page_at_end(PdfPagePaperSize::a4())
			.expect("Could not create a PDF page");

		// Attempt to split the file: it has 1 page, and thus fails
		let result = split_pdf(&pdfium, &pdf);
		assert!(
			result.is_err(),
			"split_pdf should return an error, but didn't"
		);
	}

	#[test]
	fn if_pdf_with_two_pages_then_split_returns_ok() {
		let pdfium = pdf::pdfium().expect("PDFium should be available, but isn't");

		// Create a PDF file with two pages:
		let mut pdf = pdfium
			.create_new_pdf()
			.expect("Could not create new PDF with PDFium");
		pdf.pages_mut()
			.create_page_at_end(PdfPagePaperSize::a4())
			.expect("Could not create a PDF page");
		pdf.pages_mut()
			.create_page_at_end(PdfPagePaperSize::a4())
			.expect("Could not create a PDF page");

		// Attempt to split the file: it has 2 page, and thus returns Ok
		let result = split_pdf(&pdfium, &pdf);
		assert!(
			result.is_ok(),
			"split_pdf should return Ok, but returned: {result:?}",
		);
	}

	#[test]
	fn if_pdf_with_even_amount_of_pages_then_split_returns_documents_with_same_length() {
		let pdfium = pdf::pdfium().expect("PDFium should be available, but isn't");

		// Create a PDF file with four pages:
		let mut pdf = pdfium
			.create_new_pdf()
			.expect("Could not create new PDF with PDFium");
		for _ in 0..4 {
			pdf.pages_mut()
				.create_page_at_end(PdfPagePaperSize::a4())
				.expect("Could not create a PDF page");
		}

		// Split the file:
		let (front, back) = split_pdf(&pdfium, &pdf).expect("Could not split PDF");

		// Front and back should have the same number of pages:
		assert_eq!(
			front.pages().len(),
			back.pages().len(),
			"front and back must have the same amount of pages"
		);
	}

	#[test]
	fn if_pdf_with_uneven_amount_of_pages_then_split_returns_documents_with_same_length() {
		let pdfium = pdf::pdfium().expect("PDFium should be available, but isn't");

		// Create a PDF file with five pages:
		let mut pdf = pdfium
			.create_new_pdf()
			.expect("Could not create new PDF with PDFium");
		for _ in 0..5 {
			pdf.pages_mut()
				.create_page_at_end(PdfPagePaperSize::a4())
				.expect("Could not create a PDF page");
		}

		// Split the file:
		let (front, back) = split_pdf(&pdfium, &pdf).expect("Could not split PDF");

		// Front and back should have the same amount of pages (be aligned):
		assert_eq!(
			front.pages().len(),
			back.pages().len(),
			"front and back must have the same amount of pages"
		);
	}

	#[test]
	fn if_diff_amount_of_pages_then_align_should_even_out_amount_of_pages() {
		let pdfium = pdf::pdfium().expect("PDFium should be available, but isn't");

		// Mock front and back with uneven amount of pages (back has 1 less):
		let pages_len = 3;
		let mut front = pdfium
			.create_new_pdf()
			.expect("Could not create new PDF with PDFium");
		for _ in 0..pages_len {
			front
				.pages_mut()
				.create_page_at_end(PdfPagePaperSize::a4())
				.expect("Could not create a PDF page");
		}
		let mut back = pdfium
			.create_new_pdf()
			.expect("Could not create new PDF with PDFium");
		for _ in 0..(pages_len - 1) {
			back.pages_mut()
				.create_page_at_end(PdfPagePaperSize::a4())
				.expect("Could not create a PDF page");
		}

		// Align should even the amounts out:
		let dimensions = PdfPagePaperSize::a4().as_rect();
		align_sides(&mut front, &mut back, dimensions).expect("Could not align front and back");
		assert_eq!(
			front.pages().len(),
			back.pages().len(),
			"front and back must have the same number of pages after aligning"
		);
		assert_eq!(
			front.pages().len(),
			pages_len,
			"front pages amount must remain unchanged"
		);
		assert_eq!(
			back.pages().len(),
			pages_len,
			"back pages amount must increase by one"
		);
	}

	#[test]
	fn if_same_amount_of_pages_then_align_should_not_change_amount_of_pages() {
		let pdfium = pdf::pdfium().expect("PDFium should be available, but isn't");

		// Mock front and back with same amount of pages:
		let pages_len = 2;
		let mut front = pdfium
			.create_new_pdf()
			.expect("Could not create new PDF with PDFium");
		for _ in 0..pages_len {
			front
				.pages_mut()
				.create_page_at_end(PdfPagePaperSize::a4())
				.expect("Could not create a PDF page");
		}
		let mut back = pdfium
			.create_new_pdf()
			.expect("Could not create new PDF with PDFium");
		for _ in 0..pages_len {
			back.pages_mut()
				.create_page_at_end(PdfPagePaperSize::a4())
				.expect("Could not create a PDF page");
		}

		// Align should do nothing:
		let dimensions = PdfPagePaperSize::a4().as_rect();
		align_sides(&mut front, &mut back, dimensions).expect("Could not align front and back");
		assert_eq!(
			front.pages().len(),
			back.pages().len(),
			"front and back must have the same number of pages after aligning"
		);
		assert_eq!(
			front.pages().len(),
			pages_len,
			"front pages amount must remain unchanged"
		);
		assert_eq!(
			back.pages().len(),
			pages_len,
			"back pages amount must remain unchanged"
		)
	}
}
