use std::collections::HashMap;
use std::io::Read;
use std::slice;

use windows::Win32::Graphics::Printing::{self, EnumPrintersW};

use crate::error::PrintError;
use crate::options::PrintOptions;
use crate::{CrossPlatformApi, PlatformSpecificApi, Printer};

impl CrossPlatformApi for PlatformSpecificApi {
	fn get_printers() -> Vec<Printer> {
		let mut buf_size = 0u32;
		let mut printers_len = 0u32;

		// First call to determine the buffer size (we use level 4 here, thus this is fast)
		// SAFETY: the only pointers we pass in are `buf_size` and `printers_len`, which are valid.
		let _ = unsafe {
			EnumPrintersW(
				Printing::PRINTER_ENUM_LOCAL | Printing::PRINTER_ENUM_CONNECTIONS,
				None,
				4,
				None,
				&mut buf_size,
				&mut printers_len,
			)
		};
		// Second call to populate the buffer
		// SAFETY: all pointers we pass in are valid again, and `buf` is of required size, as determined
		// by the previous call to `EnumPrintersW`.
		let mut buf = vec![0u8; buf_size as usize];
		let result = unsafe {
			EnumPrintersW(
				Printing::PRINTER_ENUM_LOCAL | Printing::PRINTER_ENUM_CONNECTIONS,
				None,
				4,
				Some(&mut buf),
				&mut buf_size,
				&mut printers_len,
			)
		};

		if result.is_err() {
			// TODO: return result
			return vec![];
		}

		unsafe {
			slice::from_raw_parts(
				buf.as_ptr() as *mut Printing::PRINTER_INFO_4W,
				printers_len as usize,
			)
		}
		.iter()
		.map(map_printer_info_4_to_printer)
		.collect::<Vec<_>>()
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

/// Converts the [`PRINTER_INFO_4W`] instance to a [`Printer`] instance.
fn map_printer_info_4_to_printer(info: &Printing::PRINTER_INFO_4W) -> Printer {
	Printer {
		identifier: unsafe { info.pPrinterName.to_string().unwrap() },
		name: unsafe { info.pPrinterName.to_string().unwrap() },
		instance: None,
		is_default: false,
		options: HashMap::new(),
	}
}
