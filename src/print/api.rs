use std::collections::HashMap;
use std::io;

use crate::error::PrintError;
use crate::options::PrintOptions;

// MARK: - Public API Methods

/// Returns a vector of available printers.
/// If no printers are available on this system, returns an empty list.
pub fn get_printers() -> Vec<Printer> {
	PlatformSpecificApi::get_printers()
}

pub fn get_printer(name: &str) -> Option<Printer> {
	PlatformSpecificApi::get_printer(name)
}

pub fn get_default_printer() -> Option<Printer> {
	PlatformSpecificApi::get_default_printer()
}

/// Prints the contents of each of the specified [`readers`].
pub fn print<I, R>(readers: I, printer: Printer, options: PrintOptions) -> Result<(), PrintError>
where
	I: IntoIterator<Item = R>,
	R: io::Read,
{
	PlatformSpecificApi::print(readers, printer, options)
}

// MARK: - Public API trait

/// A unit struct representing the current platform.
/// There should be a platform-specific implementation of [`PlatformApi`] for this struct,
/// and a module containing this implementation should be imported into the crate.
pub struct PlatformSpecificApi;
/// A trait that defines the public API of this crate.
pub trait CrossPlatformApi {
	/// See [`crate::print::get_printers()`].
	fn get_printers() -> Vec<Printer>;

	/// See [`crate::print::get_printer()`].
	fn get_printer(name: &str) -> Option<Printer>;

	/// See [`crate::print::get_default_printer()`].
	fn get_default_printer() -> Option<Printer>;

	/// See [`crate::print::print()`].
	fn print<I, R>(readers: I, printer: Printer, options: PrintOptions) -> Result<(), PrintError>
	where
		I: IntoIterator<Item = R>,
		R: io::Read;
}

// MARK: - Structs

/// A struct representing a printer.
#[derive(Debug)]
pub struct Printer {
	pub identifier: String,
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
