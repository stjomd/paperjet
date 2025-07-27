use std::collections::HashMap;

use crate::{CrossPlatformApi, TargetPlatform};

/// Returns a vector of available printers.
/// If no printers are available on this system, returns an empty list.
pub fn get_printers() -> Vec<Printer> {
	TargetPlatform::get_printers()
}

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
