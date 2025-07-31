use thiserror::Error;

#[derive(Debug, Error)]
pub enum PrintError {
	/// An error that indicates that a file name could not be extracted from a path.
	#[error("invalid path to file: {0}")]
	InvalidPath(std::path::PathBuf),

	/// An error during conversion to a C string (due to present null bytes).
	#[error("could not convert to C string: {0}")]
	StringConversion(#[from] std::ffi::NulError),

	/// An error during the reading of input files.
	#[error("could not read file: {0}")]
	FileRead(#[from] std::io::Error),

	/// An error reported by the backend API (for example, CUPS on Unix systems).
	#[error("backend error: {0}")]
	Backend(String),
}
