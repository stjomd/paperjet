use std::collections::HashMap;
use std::{ffi, ptr, slice};

use crate::print::unix::cups;
use crate::print::{CrossPlatformApi, PlatformSpecificApi, Printer};

impl CrossPlatformApi for PlatformSpecificApi {
	fn get_printers() -> Vec<Printer> {
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

	fn print_file() {
		unsafe {
			let mut ptr_dests = ptr::null_mut();
			let num_dests = cups::cupsGetDests(&mut ptr_dests);
			let chosen_dest = ptr_dests; // first FIXME

			let info = cups::cupsCopyDestInfo(cups::consts::http::CUPS_HTTP_DEFAULT, chosen_dest);

			// Set options
			let mut ptr_opts = ptr::null_mut();
			let mut num_opts = 0;
			num_opts = cups::cupsAddOption(
				cups::consts::opts::CUPS_COPIES,
				c"1".as_ptr(),
				num_opts,
				&mut ptr_opts,
			);

			// Create job
			let mut job_id = 0;
			let status = cups::cupsCreateDestJob(
				cups::consts::http::CUPS_HTTP_DEFAULT,
				chosen_dest,
				info,
				&mut job_id,
				c"TestTitlePrintrs".as_ptr(),
				num_opts,
				ptr_opts,
			);

			if status != cups::ipp_status_e::IPP_STATUS_OK {
				let message = cups::cupsLastErrorString();
				let message = ffi::CStr::from_ptr(message).to_string_lossy();
				eprintln!("Could not create print job: {message}")
			}

			eprintln!("Created job: {job_id}");

			cups::cupsFreeDests(num_dests, ptr_dests);
		}
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
			name: cstr_to_string(dest.name),
			instance,
			is_default: dest.is_default == 1,
			options,
		}
	}
}

/// Constructs an owned UTF-8 string from a valid pointer to a valid C-string.
/// Invalid characters are replaced with the replacement character.
unsafe fn cstr_to_string(ptr: *const ffi::c_char) -> String {
	unsafe { ffi::CStr::from_ptr(ptr).to_string_lossy().into_owned() }
}
