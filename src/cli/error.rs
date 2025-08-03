use std::path::PathBuf;

use colored::Colorize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CliError {
	#[error("could not find the printer with ID '{}'", .0.to_string().yellow())]
	PrinterNotFoundById(usize),

	#[error("could not find the printer with name '{}'", .0.yellow())]
	PrinterNotFoundByName(String),

	#[error("could not open file '{}': {}", .path.display().to_string().yellow(), source)]
	FileError {
		path: PathBuf,
		source: std::io::Error,
	},

	#[error("{0}")]
	ApiError(#[from] printrs::error::PrintError),
}
