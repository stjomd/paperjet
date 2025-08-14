use std::io::Read;

use crate::error::PrintError;
use crate::options::PrintOptions;
use crate::{CrossPlatformApi, PlatformSpecificApi, Printer};

impl CrossPlatformApi for PlatformSpecificApi {
	fn get_printers() -> Vec<Printer> {
		todo!("Not supported on Windows yet")
	}

	fn get_printer(_name: &str) -> Option<Printer> {
		todo!("Not supported on Windows yet")
	}

	fn get_default_printer() -> Option<Printer> {
		todo!("Not supported on Windows yet")
	}

	fn print<I, R>(_readers: I, _printer: Printer, _options: PrintOptions) -> Result<(), PrintError>
	where
		I: IntoIterator<Item = R>,
		R: Read,
	{
		todo!("Not supported on Windows yet")
	}
}
