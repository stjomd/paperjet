use anyhow::{Result, anyhow};
use pdfium_render::prelude::*;

pub mod slice;
pub mod split;

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
