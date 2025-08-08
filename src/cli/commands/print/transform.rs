use std::fs::File;

use anyhow::{Result, anyhow, bail};
use pdfium_render::prelude::*;

use crate::cli::args::PrintArgs;
use crate::cli::pdf;

/// Designates a collection of raw untransformed documents.
type OriginalDocuments<'a> = Vec<PdfDocument<'a>>;
/// Designates a collection of transformed documents.
type TransformedDocuments<'a> = Vec<PdfDocument<'a>>;
/// Designates a collection of raw transformed files, i.e. a vector of vectors of bytes.
pub type RawDocuments = Vec<Vec<u8>>;

/// Performs transformations on PDF `files` according to `args`.
///
/// If no transformations should be performed, returns the original documents as bytes.
/// Otherwise, returns transformed documents as bytes.
pub fn transform(files: Vec<File>, args: &PrintArgs) -> Result<RawDocuments> {
	let pdfium = pdf::pdfium()?;
	let mut docs = files_to_raw(&pdfium, files)?;

	if args.from.is_some() || args.to.is_some() {
		docs = slice(&pdfium, docs, args.from, args.to)?;
	}
	if args.duplex {
		docs = split_for_duplex(&pdfium, docs)?;
	}

	let raw_docs = docs
		.iter()
		.map(|d| d.save_to_bytes())
		.collect::<Result<_, _>>()?;
	Ok(raw_docs)
}

/// Slices the document, returning a new document containing the pages in the specified range.
///
/// Range indexes from 1, and is inclusive.
/// If any of `from` or `to` is `None`, they are treated as first or last page, respectively.
fn slice<'a>(
	pdfium: &'a Pdfium,
	files: OriginalDocuments<'a>,
	from: Option<PdfPageIndex>,
	to: Option<PdfPageIndex>,
) -> Result<TransformedDocuments<'a>> {
	if files.len() != 1 {
		bail!("exactly one file must be specified to slice a document")
	}
	let source = extract_first(files);
	let sliced = pdf::slice::slice_document(pdfium, &source, from, to)?;
	Ok(vec![sliced])
}

/// Splits the document into two, for each of the paper sides (front and back).
fn split_for_duplex<'a>(
	pdfium: &'a Pdfium,
	files: OriginalDocuments<'a>,
) -> Result<TransformedDocuments<'a>> {
	if files.len() != 1 {
		bail!("exactly one file must be specified to print in duplex mode")
	}
	let source = extract_first(files);
	let (front, back) = pdf::split::split_pdf(pdfium, &source)?;
	Ok(vec![front, back])
}

/// Reads the files into buffers of bytes.
fn files_to_raw<'a>(pdfium: &'a Pdfium, files: Vec<File>) -> Result<OriginalDocuments<'a>> {
	files
		.into_iter()
		.map(|file| match pdfium.load_pdf_from_reader(file, None) {
			Ok(document) => Ok(document),
			Err(e) => Err(anyhow!("could not read from file: {e}")),
		})
		.collect::<Result<OriginalDocuments>>()
}

/// Consumes the vector and returns the first value as an owned value.
/// Panics if the vector has no elements.
fn extract_first<T>(mut vec: Vec<T>) -> T {
	vec.remove(0)
}
