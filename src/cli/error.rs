use std::path::PathBuf;

use colored::Colorize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CliError {
	/// An error indicating that a printer could not be found by a criteria.
	#[error("could not find a printer by criteria: '{}'", .0.yellow())]
	PrinterNotFound(String),

	/// An error indicating that a printer could not be found by an ID
	/// corresponding to the `list` command.
	#[error("could not find a printer by ID: '{}'", .0.to_string().yellow())]
	PrinterNotFoundById(usize),

	/// An error indicating that a printer could not be found by a name.
	#[error("could not find a printer by name: '{}'", .0.yellow())]
	PrinterNotFoundByName(String),

	/// An error indicating that a file could not be opened.
	#[error("could not open file '{}': {}", .path.display().to_string().yellow(), source)]
	FileError {
		path: PathBuf,
		source: std::io::Error,
	},

	/// An error coming from the API.
	#[error("{0}")]
	ApiError(#[from] printrs::error::PrintError),
}
