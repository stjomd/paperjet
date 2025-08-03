#![cfg(target_family = "unix")]

mod utils;

use crate::utils::unixutils::FakePrinter;

#[test]
fn get_printers_returns_correct_information_of_printer() {
	// Create a fake printer (removed from lpstat on drop)
	let fake = FakePrinter::try_new().expect("Could not create a fake printer");
	// Get information
	let printers = printrs::get_printers();

	// Only one printer with the fake's name should exist
	let fake_printer = printers.iter().find(|p| p.name == fake.name);
	assert!(fake_printer.is_some());

	// The fake printer should have correct information
	let fake_printer = fake_printer.unwrap();
	assert_eq!(fake_printer.name, fake.name);
	assert_eq!(
		fake_printer
			.get_option("printer-info")
			.expect("option printer-info is not present"),
		&fake.name
	);
	assert_eq!(
		fake_printer
			.get_option("device-uri")
			.expect("option device-uri is not present"),
		&fake.device_uri
	);
}
