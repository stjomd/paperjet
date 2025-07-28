use colored::Colorize;

use crate::cli::common::get_sorted_printers;

/// The `list` command.
pub fn list() {
	for (i, printer) in get_sorted_printers().iter().enumerate() {
		let index = i + 1;
		let name = printer.get_option("printer-info").unwrap_or(&printer.name);

		let line = format!("{index}. {name}");
		if printer.is_default {
			println!("{}", (line + " (default)").bold().cyan());
		} else {
			println!("{}", line.bold());
		}
	}
}
