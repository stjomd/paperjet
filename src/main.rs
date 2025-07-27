use std::cmp::Ordering;
use std::process;

use colored::Colorize;
use printrs::Printer;

use crate::args::{Args, Command, DisplayArgs};

mod args;

fn main() {
	let args = Args::parse();
	match args.command {
		Command::List => list(),
		Command::Display(d_args) => display(d_args),
	}
}

fn list() {
	let mut printers = printrs::get_printers();
	printers.sort_by(|a, b| {
		if a.is_default {
			return Ordering::Less;
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

fn display(args: DisplayArgs) {
	let mut printers = printrs::get_printers();
	printers.sort_by(|a, b| {
		if a.is_default {
			return Ordering::Less;
		}
		a.name.cmp(&b.name)
	});

	let filtered_printers = printers
		.iter()
		.enumerate()
		.filter(|(i, _)| args.id == (*i + 1))
		.map(|(_, p)| p)
		.collect::<Vec<&Printer>>();

	let printer = filtered_printers.first();
	let Some(printer) = printer else {
		println!("No printer with ID {} was found.", args.id);
		process::exit(1);
	};

	println!("{}\n", printer.get_human_name().bold());

	println!("Identifier: {}", printer.name);
	println!(
		"Model: {}",
		printer
			.get_option("printer-make-and-model")
			.unwrap_or(&"unknown".to_owned())
	);
	println!("Default: {}", printer.is_default);

	let state = printer
		.get_option("printer-state")
		.and_then(|value| match value.as_str() {
			"3" => Some("idle"),
			"4" => Some("printing"),
			"5" => Some("stopped"),
			_ => None,
		});
	println!("State: {}", state.unwrap_or("unknown"));

	println!(
		"Accepting jobs: {}",
		printer
			.get_option("printer-is-accepting-jobs")
			.unwrap_or(&"unknown".to_owned())
	);

	let marker_level = get_marker_level(printer);
	let marker_level = if let Some(percentage) = marker_level {
		percentage.to_string()
	} else {
		"unknown".to_owned()
	};
	println!("Ink level: {marker_level}%");

	if args.options {
		let header = format!("Options ({}):", printer.options.len());
		println!("\n{}", header.bold());
		display_options(printer);
	}
}

fn display_options(printer: &Printer) {
	let mut entries = printer.options.iter().collect::<Vec<(&String, &String)>>();
	entries.sort_by_key(|entry| entry.0);

	for (name, value) in entries {
		println!("{name}: {value}");
	}
}

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
