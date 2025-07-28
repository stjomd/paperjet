use std::cmp;

use colored::Colorize;

/// The `list` command.
pub fn list() {
	let mut printers = printrs::get_printers();
	printers.sort_by(|a, b| {
		if a.is_default {
			return cmp::Ordering::Less;
		}
		a.name.cmp(&b.name)
	});

	for (i, printer) in printers.iter().enumerate() {
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
