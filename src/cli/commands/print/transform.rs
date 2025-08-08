use std::fs::File;
use std::io::{Cursor, Read};

use anyhow::{Result, anyhow, bail};
use pdfium_render::prelude::*;

use crate::cli::args::PrintArgs;
use crate::cli::pdf;

/// Designates a collection of raw untransformed files, i.e. a vector of files.
type OriginalFiles = Vec<Vec<u8>>;
/// Designates a collection of raw transformed files, i.e. a vector of files.
pub type TransformedFiles = Vec<Vec<u8>>;

/// Performs transformations on PDF `files` according to `args`.
///
/// If no transformations should be performed, returns the original documents as bytes.
/// Otherwise, returns transformed documents as bytes.
pub fn transform(files: Vec<File>, args: &PrintArgs) -> Result<TransformedFiles> {
	let mut raw_files = files_to_raw(files)?;
	let pdfium = pdf::pdfium()?;

	if args.from.is_some() || args.to.is_some() {
		raw_files = slice(&pdfium, raw_files, args.from, args.to)?;
	}
	if args.duplex {
		raw_files = split_for_duplex(&pdfium, raw_files)?;
	}

	Ok(raw_files)
}

/// Slices the document, returning a new document containing the pages in the specified range.
///
/// Range indexes from 1, and is inclusive.
/// If any of `from` or `to` is `None`, they are treated as first or last page, respectively.
fn slice(
	pdfium: &Pdfium,
	files: OriginalFiles,
	from: Option<PdfPageIndex>,
	to: Option<PdfPageIndex>,
) -> Result<TransformedFiles> {
	if files.len() != 1 {
		bail!("exactly one file must be specified to slice a document")
	}
	let raw_file = Cursor::new(&files[0]);

	let source = pdfium.load_pdf_from_reader(raw_file, None)?;
	let sliced = pdf::slice::slice_document(pdfium, &source, from, to)?;

	Ok(vec![sliced.save_to_bytes()?])
}

/// Splits the document into two, for each of the paper sides (front and back).
fn split_for_duplex(pdfium: &Pdfium, files: OriginalFiles) -> Result<TransformedFiles> {
	if files.len() != 1 {
		bail!("exactly one file must be specified to print in duplex mode")
	}
	let raw_file = Cursor::new(&files[0]);

	let source = pdfium.load_pdf_from_reader(raw_file, None)?;
	let (front, back) = pdf::split::split_pdf(pdfium, &source)?;

	Ok(vec![front.save_to_bytes()?, back.save_to_bytes()?])
}

/// Reads the files into buffers of bytes.
fn files_to_raw(files: Vec<File>) -> Result<OriginalFiles> {
	files
		.into_iter()
		.map(|mut file| {
			let mut buffer = Vec::new();
			match file.read_to_end(&mut buffer) {
				Ok(_) => Ok(buffer),
				Err(e) => Err(anyhow!("could not read from file: {e}")),
			}
		})
		.collect::<Result<OriginalFiles>>()
}
