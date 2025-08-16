use std::ptr;

use windows::Win32::Graphics::Printing;
use windows::core::{HSTRING, PCWSTR};

use crate::error::PrintError;

pub struct PrinterHandle(*mut Printing::PRINTER_HANDLE);
impl PrinterHandle {
	pub fn try_new(name: &str) -> Result<Self, PrintError> {
		let handle = ptr::null_mut();

		let h_name = HSTRING::from(name);
		let name = PCWSTR::from_raw(h_name.as_ptr());

		let result = unsafe { Printing::OpenPrinterW(name, handle, None) };
		if let Err(e) = result {
			return Err(PrintError::Backend(format!("{e}")));
		}

		Ok(Self(handle))
	}
}
impl Drop for PrinterHandle {
	fn drop(&mut self) {
		let _ = unsafe { Printing::ClosePrinter(*self.0) };
	}
}
