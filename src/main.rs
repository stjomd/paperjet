use std::cmp::Ordering;

use colored::Colorize;

use crate::args::{Args, ListArgs};

mod args;

fn main() {
	let args = Args::parse();
	match args.command {
		args::Command::List(list_args) => list(list_args),
	}
}

fn list(_: ListArgs) {
	let mut printers = printrs::get_printers();
	printers.sort_by(|a, b| {
		if a.is_default {
			return Ordering::Less;
		}
		a.name.cmp(&b.name)
	});

	println!("Available printers:");
	for (i, printer) in printers.iter().enumerate() {
		let index = i + 1;
		let name = printer.get_option("printer-info").unwrap_or(&printer.name);

		let line = format!("{}. {}", index, name);
		if printer.is_default {
			println!("{}", (line + " (default)").bold().blue())
		} else {
			println!("{}", line.bold())
		}
	}
}
