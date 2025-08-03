use thiserror::Error;

#[derive(Debug, Error)]
pub enum PrintError {
	/// An error that indicates that no printer with the given name could be found.
	#[error("could not find printer: {0}")]
	PrinterNotFound(String),

	/// An error that indicates that there are no printers to work with.
	#[error("no printers available")]
	NoPrinters,

	/// An error during the reading of input files.
	#[error("could not read file: {0}")]
	FileRead(#[from] std::io::Error),

	/// An error during conversion to a C string (due to present null bytes).
	#[error("could not convert to C string: {0}")]
	StringConversion(#[from] std::ffi::NulError),

	/// An error indicating that necessary information for a process could not be retrieved.
	#[error("could not retrieve necessary information: {0}")]
	NecessaryInformationMissing(String),

	/// An error that indicates that an option validation with the printer failed.
	#[error("printer does not support option: {name} = {value}")]
	UnsupportedOption { name: String, value: String },

	/// An error reported by the backend API (for example, CUPS on Unix systems).
	#[error("{0}")]
	Backend(String),
}
