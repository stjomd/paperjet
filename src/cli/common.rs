use std::cmp;

use printrs::Printer;

/// Returns printers in a sorted order.
pub fn get_sorted_printers() -> Vec<Printer> {
	let mut printers = printrs::get_printers();
	printers.sort_by(|a, b| {
		if a.is_default {
			return cmp::Ordering::Less;
		}
		a.name.cmp(&b.name)
	});
	printers
}
