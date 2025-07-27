#![cfg(target_family = "unix")]

mod utils;

use crate::utils::*;
use printrs::Printer;

#[test]
fn get_printers_returns_correct_information_of_printer() {
	// Create a dummy printer (removed from lpstat on drop)
	let dummy = DummyPrinter::try_new().expect("Could not create dummy printer");
	// Get information
	let printers = printrs::get_printers();

	// Only one printer with the dummy's name should exist
	let dummy_printers = printers
		.iter()
		.filter(|p| p.name == dummy.name)
		.collect::<Vec<&Printer>>();
	assert_eq!(dummy_printers.len(), 1);

	// The dummy printer should have correct information
	let dummy_printer = dummy_printers[0];
	assert_eq!(dummy_printer.name, dummy.name);
	assert_eq!(
		dummy_printer
			.get_option("printer-info")
			.expect("option printer-info is not present"),
		&dummy.name
	);
	assert_eq!(
		dummy_printer
			.get_option("device-uri")
			.expect("option device-uri is not present"),
		&dummy.device_uri
	);
}
