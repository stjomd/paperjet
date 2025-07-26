macro_rules! assert_matches {
	($e:expr, $p:pat) => {
		assert!(matches!($e, $p))
	};
	($e:expr, $p:pat, $m:expr) => {
		assert!(matches!($e, $p), $m)
	};
}

#[test]
fn get_printers_options_are_correct() {
	// We have no control over the input here, and depend on whether the system has any printers available.
	let printers = printrs::get_printers();
	if printers.is_empty() {
		// If there are no printers, we can't test anything.
		return;
	}
	let printer = printers
		.first()
		.expect("the first element should be present in non-empty vec");

	// Several options are predefined: https://www.cups.org/doc/cupspm.html#basic-destination-information
	// They're not guaranteed to be set - thus this test only makes sense on a machine where they do appear.

	// We expect these to exist:
	let _ = printer
		.get_option("printer-info")
		.expect("option printer-info is not present");
	let _ = printer
		.get_option("printer-make-and-model")
		.expect("option printer-make-and-model is not present");

	// Values of these are restricted:
	let printer_is_accepting_jobs = printer.get_option("printer-is-accepting-jobs");
	if let Some(printer_is_accepting_jobs) = printer_is_accepting_jobs {
		assert_matches!(printer_is_accepting_jobs.as_str(), "true" | "false");
	}
	let printer_is_shared = printer.get_option("printer-is-shared");
	if let Some(printer_is_shared) = printer_is_shared {
		assert_matches!(printer_is_shared.as_str(), "true" | "false");
	}
	let printer_state = printer.get_option("printer-state");
	if let Some(printer_state) = printer_state {
		assert_matches!(printer_state.as_str(), "3" | "4" | "5");
	}
}
