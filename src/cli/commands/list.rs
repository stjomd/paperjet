use anyhow::Result;
use colored::Colorize;

use crate::cli::common::get_sorted_printers;
use crate::cli::snapshot;

/// The `list` command.
pub fn list() -> Result<()> {
	let printers = get_sorted_printers();
	for (i, printer) in printers.iter().enumerate() {
		let index = i + 1;
		let name = printer.get_human_name();

		let line = format!("{index}. {name}");
		if printer.is_default {
			println!("{}", (line + " (default)").bold().cyan());
		} else {
			println!("{}", line.bold());
		}
	}
	snapshot::printers::save(&printers);
	Ok(())
}
