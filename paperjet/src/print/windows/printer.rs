use windows::Win32::Graphics::Printing;

use crate::error::PrintError;
use crate::windows::str::WideString;

/// A struct representing a handle to a Windows printer.
/// Internally, it wraps [`Printing::PRINTER_HANDLE`] and follows RAII
/// (the handle will be closed once the instance goes out of scope).
pub struct PrinterHandle(Printing::PRINTER_HANDLE);
impl PrinterHandle {
	/// Obtains a handle to the printer with the specified name.
	/// Returns `Err` if the handle could not be obtained. 
	pub fn try_new(name: &str) -> Result<Self, PrintError> {
		let mut handle = Default::default();
		let w_name = WideString::from(name);

		let result = unsafe { Printing::OpenPrinterW(w_name.as_pcwstr(), &mut handle, None) };
		if let Err(e) = result {
			let msg = format!("could not obtain handle for '{name}': {e}");
			return Err(PrintError::Backend(msg));
		}

		Ok(Self(handle))
	}
	/// Returns the underlying [`Printing::PRINTER_HANDLE`] behind this instance.
	pub fn unwrap(&self) -> Printing::PRINTER_HANDLE {
		self.0
	}
}

impl Drop for PrinterHandle {
	fn drop(&mut self) {
		let _ = unsafe { Printing::ClosePrinter(self.0) };
	}
}
