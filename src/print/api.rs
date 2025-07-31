use std::collections::HashMap;
use std::path;

use crate::error::PrintError;

// MARK: - Public API Methods

/// Returns a vector of available printers.
/// If no printers are available on this system, returns an empty list.
pub fn get_printers() -> Vec<Printer> {
	PlatformSpecificApi::get_printers()
}

/// Prints a specified file.
pub fn print_files(paths: &[path::PathBuf]) -> Result<(), PrintError> {
	PlatformSpecificApi::print_files(paths)
}

// MARK: - Public API trait

/// A unit struct representing the current platform.
/// There should be a platform-specific implementation of [`PlatformApi`] for this struct,
/// and a module containing this implementation should be imported above.
pub struct PlatformSpecificApi;
/// A trait that defines the public API of this crate.
pub trait CrossPlatformApi {
	/// See [`crate::print::get_printers()`].
	fn get_printers() -> Vec<Printer>;
	/// See [`crate::print::print_files()`].
	fn print_files(paths: &[path::PathBuf]) -> Result<(), PrintError>;
}

// MARK: - Structs

/// A struct representing a printer.
#[derive(Debug)]
pub struct Printer {
	pub name: String,
	pub instance: Option<String>,
	pub is_default: bool,
	pub options: HashMap<String, String>,
}
impl Printer {
	/// Returns the value of the option with the specified name.
	pub fn get_option(&self, name: &str) -> Option<&String> {
		self.options.get(name)
	}
	/// Returns a human-friendly name of this printer.
	/// If no such name is available, returns the regular name (identifier).
	pub fn get_human_name(&self) -> &String {
		self.get_option("printer-info").unwrap_or(&self.name)
	}
}
