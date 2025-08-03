use std::cmp::Ordering;

use printrs::Printer;
use printrs::options::{CopiesInt, NumberUpInt, PrintOptions};

use crate::cli::args::PrintArgs;
use crate::cli::snapshot;

/// Returns printers in a sorted order.
pub fn get_sorted_printers() -> Vec<Printer> {
	let mut printers = printrs::get_printers();
	printers.sort_by(|a, b| {
		if a.is_default {
			return Ordering::Less;
		}
		a.name.cmp(&b.name)
	});
	printers
}

/// Retrieves the printer corresponding to the specified `id`.
/// The ID refers to the position in the list output by the `list` command.
pub fn get_printer_by_id(id: usize) -> Option<Printer> {
	if id < 1 {
		return None; // `list` counts from 1
	}
	let index = id - 1;
	get_printer_by_id_from_snapshot(index).or_else(|| get_printer_by_id_from_api(index))
}
/// Retrieves the printer at the specified index in the snapshot, if present.
fn get_printer_by_id_from_snapshot(index: usize) -> Option<Printer> {
	let snapshot = snapshot::printers::open()?;
	let entry = snapshot.get(index)?;
	printrs::get_printer(&entry.identifier)
}
/// Retrieves all printers from backend, then returns the printer with the specified index,
/// if present.
fn get_printer_by_id_from_api(index: usize) -> Option<Printer> {
	let printers = get_sorted_printers();
	snapshot::printers::save(&printers);
	printers.into_iter().nth(index)
}

/// Retrieves the printer corresponding to the specified `name`.
/// The name is matched against the printer's identifier, name, and human-friendly name.
pub fn get_printer_by_name(name: &str) -> Option<Printer> {
	get_printer_by_name_from_snapshot(name).or_else(|| get_printer_by_name_from_api(name))
}
/// Retrieves the printer corresponding to the specified `name` from the snapshot.
fn get_printer_by_name_from_snapshot(name: &str) -> Option<Printer> {
	let snapshot = snapshot::printers::open()?;
	let entry = snapshot.iter().find(|snap| snap.identifier == name);
	let entry = entry.or_else(|| snapshot.iter().find(|snap| snap.human_name == name))?;
	printrs::get_printer(&entry.identifier)
}
// Retrieves the printer corresponding to the specified `name` from the API.
fn get_printer_by_name_from_api(name: &str) -> Option<Printer> {
	let printers = printrs::get_printers();
	printers
		.into_iter()
		.find(|p| p.identifier == name || p.name == name || p.get_human_name() == name)
}

impl From<PrintArgs> for PrintOptions {
	fn from(value: PrintArgs) -> PrintOptions {
		PrintOptions {
			copies: value.copies.map(CopiesInt::from),
			finishings: value.finishings,
			media_size: value.size,
			media_source: value.source,
			media_type: value.media_type,
			number_up: value.number_up.map(NumberUpInt::from),
			orientation: value.orientation,
			color_mode: value.color_mode,
			quality: value.quality,
			sides_mode: value.sides_mode,
		}
	}
}
