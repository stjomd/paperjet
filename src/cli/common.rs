use std::{cmp, ffi};

use printrs::Printer;
use printrs::options::{CopiesInt, NumberUpInt, PrintOptions};

use crate::cli::args::PrintArgs;

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

impl From<PrintArgs> for PrintOptions {
	fn from(value: PrintArgs) -> PrintOptions {
		PrintOptions {
			copies: value.copies.map(|c| CopiesInt(c as ffi::c_int)),
			finishings: value.finishings,
			media_size: value.size,
			media_source: value.source,
			media_type: value.media_type,
			number_up: value.number_up.map(|u| NumberUpInt(u as ffi::c_int)),
			orientation: value.orientation,
			color_mode: value.color_mode,
			quality: value.quality,
			sides_mode: value.sides_mode,
		}
	}
}
