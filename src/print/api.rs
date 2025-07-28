use std::collections::HashMap;

// MARK: - Public API Methods

/// Returns a vector of available printers.
/// If no printers are available on this system, returns an empty list.
pub fn get_printers() -> Vec<Printer> {
	PlatformSpecificApi::get_printers()
}

// MARK: - Public API trait

/// A unit struct representing the current platform.
/// There should be a platform-specific implementation of [`PlatformApi`] for this struct,
/// and a module containing this implementation should be imported above.
pub struct PlatformSpecificApi;
/// A trait that defines the public API of this crate.
pub trait CrossPlatformApi {
	/// See [`get_printers()`].
	fn get_printers() -> Vec<Printer>;
}

// MARK: - Structs

/// A struct representing a printer.
#[allow(dead_code)]
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
