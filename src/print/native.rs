use std::collections::HashMap;
use std::{ffi, ptr, slice};

use crate::print::cups;

/// A struct representing a printer.
#[allow(dead_code)]
#[derive(Debug)]
pub struct Printer {
	name: ffi::CString,
	instance: Option<String>,
	is_default: bool,
	options: HashMap<String, String>,
}

/// Returns a vector of available printers.
/// If no printers are available on this system, returns an empty list.
pub fn get_printers() -> Vec<Printer> {
	unsafe {
		let mut ptr_dests = ptr::null_mut();
		let num_dests = cups::cupsGetDests(&mut ptr_dests);

		let dests = if num_dests > 0 {
			slice::from_raw_parts(ptr_dests, num_dests as usize)
		} else {
			return vec![];
		};
		let printers = dests.iter().map(|dest| map_dest_to_printer(dest)).collect();

		cups::cupsFreeDests(num_dests, ptr_dests);
		printers
	}
}

/// Maps an instance of [`cups::cups_dest_t`] to a [`Printer`].
/// The argument's pointers must all be valid.
unsafe fn map_dest_to_printer(dest: &cups::cups_dest_t) -> Printer {
	unsafe {
		let options = slice::from_raw_parts(dest.options, dest.num_options as usize)
			.iter()
			.map(|opt| (cstr_to_string(opt.name), cstr_to_string(opt.value)))
			.collect::<HashMap<String, String>>();

		let instance = if !dest.instance.is_null() {
			Some(cstr_to_string(dest.instance))
		} else {
			None
		};

		Printer {
			name: cstr_to_cstring(dest.name),
			instance,
			is_default: dest.is_default == 1,
			options,
		}
	}
}

/// Constructs an owned UTF-8 string from a valid pointer to a valid C-string.
/// Invalid characters are replaced with the replacement character.
unsafe fn cstr_to_string(ptr: *const ffi::c_char) -> String {
	unsafe { ffi::CStr::from_ptr(ptr).to_string_lossy().to_string() }
}

/// Constructs an owned C-string from a valid pointer to a valid C-string.
unsafe fn cstr_to_cstring(ptr: *const ffi::c_char) -> ffi::CString {
	unsafe { ffi::CStr::from_ptr(ptr).to_owned() }
}
