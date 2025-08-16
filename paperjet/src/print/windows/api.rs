use std::collections::HashMap;
use std::io::Read;
use std::slice;

use windows::Win32::Graphics::Printing;
use windows::core::PWSTR;

use crate::error::PrintError;
use crate::options::PrintOptions;
use crate::windows::printer::PrinterHandle;
use crate::{PaperjetApi, Platform, Printer};

impl PaperjetApi for Platform {
	fn get_printers() -> Result<Vec<Printer>, PrintError> {
		let mut buf_size = 0;
		let mut printers_len = 0;

		// First call to determine the buffer size (we use level 4 here, thus this is fast)
		// SAFETY: the only pointers we pass in are `buf_size` and `printers_len`, which are valid.
		let _ = unsafe {
			Printing::EnumPrintersW(
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
			Printing::EnumPrintersW(
				Printing::PRINTER_ENUM_LOCAL | Printing::PRINTER_ENUM_CONNECTIONS,
				None,
				4,
				Some(&mut buf),
				&mut buf_size,
				&mut printers_len,
			)
		};
		if let Err(e) = result {
			return Err(PrintError::Backend(format!("{e}")));
		}

		// SAFETY: `buf` has been successfully written to by `EnumPrintersW` and has `printers_len`
		// elements.
		unsafe {
			slice::from_raw_parts(
				buf.as_ptr() as *mut Printing::PRINTER_INFO_4W,
				printers_len as usize,
			)
			.iter()
			.map(|info4| map_printer_info_4_to_printer(info4))
			.collect::<Result<Vec<_>, _>>()
		}
	}

	fn get_printer(name: &str) -> Option<Printer> {
		if let Some(default_printer) = crate::get_default_printer() {
			if default_printer.identifier == name {
				return Some(default_printer);
			}
		}

		// Just check if we can obtain a handle to a printer with this name.
		if PrinterHandle::try_new(name).is_err() {
			return None;
		}

		Some(Printer {
			identifier: name.to_owned(),
			name: name.to_owned(),
			instance: None,
			is_default: false,
			options: HashMap::new(),
		})
	}

	fn get_default_printer() -> Option<Printer> {
		let mut len = 0;

		// First call to determine the string length
		// SAFETY: the first argument can be a null pointer, and the second is a valid pointer to an
		// integer.
		let _ = unsafe { Printing::GetDefaultPrinterW(None, &mut len) };

		// Second call to fill the buffer with the string bytes
		// SAFETY: both pointers are valid and will be populated with valid contents by Windows.
		let mut buf = vec![0u16; len as usize];
		let status =
			unsafe { Printing::GetDefaultPrinterW(Some(PWSTR(buf.as_mut_ptr())), &mut len) };
		if !status.as_bool() {
			return None;
		}

		let name = String::from_utf16_lossy(&buf[..(buf.len() - 1)]);
		Some(Printer {
			identifier: name.clone(),
			name,
			instance: None,
			is_default: true,
			options: HashMap::new(),
		})
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
///
/// # Safety
/// `info` must be a valid reference to a valid `PRINTER_INFO_4W`.
unsafe fn map_printer_info_4_to_printer(
	info: &Printing::PRINTER_INFO_4W,
) -> Result<Printer, PrintError> {
	// SAFETY: info is a valid `PRINTER_INFO_4W` instance, as per the precondition.
	let identifier = unsafe { info.pPrinterName.to_string()? };

	if let Some(default_printer) = crate::get_default_printer() {
		if default_printer.identifier == identifier {
			return Ok(default_printer);
		}
	};

	Ok(Printer {
		identifier: identifier.clone(),
		name: identifier,
		instance: None,
		is_default: false,
		options: HashMap::new(),
	})
}
