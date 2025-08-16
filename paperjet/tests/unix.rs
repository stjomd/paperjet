#![cfg(unix)]

mod utils;

use paperjet::options::PrintOptions;

use crate::utils::unixutils::FakePrinter;

#[test]
fn if_many_printers_exist_then_get_printers_returns_all() {
	// Create several fake printers:
	let fakes = [
		FakePrinter::try_new(false).expect("Could not create a fake printer"),
		FakePrinter::try_new(false).expect("Could not create a fake printer"),
		FakePrinter::try_new(false).expect("Could not create a fake printer"),
	];
	// Get all printers:
	let printers = paperjet::get_printers().expect("Could not get printers");

	// Each of the fake printers must be present in the vector returned by get_printers.
	// Since other printers may be present on the test runner's system, we can't make any other
	// assumptions about the returned vector.
	for fake in fakes {
		let printer = printers.iter().find(|p| p.identifier == fake.name);
		assert!(
			printer.is_some(),
			"Fake printer {} was not present in the vector",
			fake.name
		);
	}
}

#[test]
fn get_printer_returns_correct_information_of_printer() {
	// Create a fake printer:
	let fake = FakePrinter::try_new(false).expect("Could not create a fake printer");
	// Get information:
	let printer = paperjet::get_printer(&fake.name).expect("Could not find the fake printer");

	// The fake printer should have correct information:
	assert_eq!(printer.name, fake.name);
	assert_eq!(
		printer
			.get_option("printer-info")
			.expect("option printer-info is not present"),
		&fake.name
	);
	assert_eq!(
		printer
			.get_option("device-uri")
			.expect("option device-uri is not present"),
		&fake.device_uri
	);
}

#[test]
fn if_printer_exists_then_get_printer_returns_correct_printer() {
	// Create a fake printer:
	let fake = FakePrinter::try_new(false).expect("Could not create a fake printer");
	// Get printer with the fake's name:
	let printer = paperjet::get_printer(&fake.name).expect("Could not find printer by name");

	// get_printer should return the correct printer with the same name:
	assert_eq!(
		fake.name, printer.identifier,
		"printer's name should be '{}', was: '{}'",
		fake.name, printer.identifier
	);
}

#[test]
fn if_printer_not_exists_then_get_printer_returns_none() {
	// Create a fake printer and clone its name; fake is dropped when going out of scope,
	// removing it from the system:
	let fake_name = {
		let fake = FakePrinter::try_new(false).expect("Could not create a fake printer");
		fake.name.clone()
	};
	// Get printer with the fake's name that does not exist anymore:
	let printer = paperjet::get_printer(&fake_name);
	assert!(
		printer.is_none(),
		"Printer was dropped/removed but was found"
	);
}

#[test]
fn if_printer_accepts_jobs_then_print_returns_unit() {
	// Create fake printer that accepts jobs:
	let fake = FakePrinter::try_new(true).expect("Could not create a fake printer");
	// Create a mock document:
	let document = [0u8; 1024];

	// Get the printer:
	let printer = paperjet::get_printer(&fake.name).expect("Could not find the fake printer");
	// Submit print job:
	let result = paperjet::print([&document[..]], printer, PrintOptions::default());
	assert!(
		result.is_ok(),
		"Print job should be submitted successfully, but wasn't"
	);
}

#[test]
fn if_printer_not_accepts_jobs_then_print_returns_err() {
	// Create fake printer that doesn't accept jobs:
	let fake = FakePrinter::try_new(false).expect("Could not create a fake printer");
	// Create a mock document:
	let document = [0u8; 1024];

	// Get the printer:
	let printer = paperjet::get_printer(&fake.name).expect("Could not find the fake printer");
	// Submit print job:
	let result = paperjet::print([&document[..]], printer, PrintOptions::default());
	assert!(result.is_err(), "Print job should not be accepted, but was");
}

#[test]
fn if_printer_no_longer_exists_then_print_returns_err() {
	// Create a fake printer:
	let fake = FakePrinter::try_new(false).expect("Could not create a fake printer");
	// Create a mock document:
	let document = [0u8; 1024];
	// Get the printer:
	let printer = paperjet::get_printer(&fake.name).expect("Could not find the fake printer");
	// Remove printer:
	drop(fake);

	// Submit print job:
	let result = paperjet::print([&document[..]], printer, PrintOptions::default());
	assert!(result.is_err(), "Print job should not be accepted, but was");
}
