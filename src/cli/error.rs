use std::path::PathBuf;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum CliError {
	#[error("could not find printer with ID {0}")]
	PrinterNotFound(usize),

	#[error("could not open file '{path}': {source}")]
	FileError {
		path: PathBuf,
		source: std::io::Error,
	},

	#[error("{0}")]
	ApiError(#[from] printrs::error::PrintError),
}
