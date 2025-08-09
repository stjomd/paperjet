use std::collections::HashMap;

use anyhow::{Result, anyhow};
use colored::Colorize;
use paperjet::Printer;

use crate::cli::args::DisplayArgs;
use crate::cli::common;

type KeyValueMap = HashMap<String, Option<String>>;

/// The `display` command.
pub fn display(args: DisplayArgs) -> Result<()> {
	let printer = get_printer_by_criteria(&args.criteria)?;
	println!("{}\n", printer.get_human_name().bold());

	let info = collect_information(&printer);
	print_key_value_pairs(&info);

	if args.options {
		let options = collect_options(&printer);
		let header = format!("Options ({}):", printer.options.len());
		println!("\n{}", header.bold());
		print_key_value_pairs(&options);
	}
	Ok(())
}

/// Retrieves the printer by specified criteria, which is either the numerical ID or a name.
fn get_printer_by_criteria(criteria: &str) -> Result<Printer> {
	let mut printer = None;
	if let Ok(id) = criteria.parse::<usize>() {
		printer = common::get_printer_by_id(id);
	};
	if printer.is_none() {
		printer = common::get_printer_by_name(criteria);
	}
	printer.ok_or_else(|| {
		anyhow!(
			"could not find a printer by criteria: '{}'",
			criteria.yellow()
		)
	})
}

/// Collects basic printer information into a map.
fn collect_information(printer: &Printer) -> KeyValueMap {
	let mut map = HashMap::new();

	map.insert("Identifier".to_owned(), Some(printer.name.to_owned()));

	map.insert(
		"Model".to_owned(),
		printer
			.get_option("printer-make-and-model")
			.map(|value| value.to_owned()),
	);

	map.insert(
		"State".to_owned(),
		printer
			.get_option("printer-state")
			.and_then(|value| match value.as_str() {
				"3" => Some("idle"),
				"4" => Some("printing"),
				"5" => Some("stopped"),
				_ => None,
			})
			.map(|value| value.to_owned()),
	);

	map.insert(
		"Accepting jobs".to_owned(),
		printer
			.get_option("printer-is-accepting-jobs")
			.map(|value| value.to_owned()),
	);

	map.insert(
		"Ink level".to_owned(),
		get_marker_level(printer).map(|percentage| percentage.to_string() + "%"),
	);

	map
}

/// Collects printer options into the map.
fn collect_options(printer: &Printer) -> KeyValueMap {
	printer
		.options
		.iter()
		.map(|(k, v)| (k.to_owned(), Some(v.to_owned())))
		.collect::<HashMap<_, _>>()
}

/// Prints the map in alphabetical order of the keys.
fn print_key_value_pairs(map: &KeyValueMap) {
	let mut entries = map.iter().collect::<Vec<_>>();
	entries.sort_by_key(|entry| entry.0);

	for (key, _value) in entries {
		match _value.as_ref() {
			None => println!("{key}: {}", "<unknown>".italic()),
			Some(value) if value.is_empty() => println!("{key}: {}", "<unknown>".italic()),
			Some(value) => println!("{key}: {value}"),
		}
	}
}

/// Calculates the percentage of the marker level in a printer.
fn get_marker_level(printer: &Printer) -> Option<u8> {
	let marker_levels = printer.get_option("marker-levels")?.parse::<u32>().ok()?;

	let marker_low_levels = printer
		.get_option("marker-low-levels")?
		.parse::<u32>()
		.unwrap_or(0);
	let marker_high_levels = printer
		.get_option("marker-high-levels")?
		.parse::<u32>()
		.unwrap_or(100);

	let percentage = ((marker_levels - marker_low_levels) as f64)
		/ ((marker_high_levels - marker_low_levels) as f64)
		* 100.0;
	Some(percentage as u8)
}
