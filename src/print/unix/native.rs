use std::collections::HashMap;
use std::io::Read;
use std::os::unix::ffi::OsStrExt;
use std::{ffi, fs, path, ptr, slice};

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

	fn print_file(path: &path::Path) {
		unsafe {
			let mut ptr_dests = ptr::null_mut();
			let num_dests = cups::cupsGetDests(&mut ptr_dests);
			let chosen_dest = ptr_dests; // first FIXME

			let info = cups::cupsCopyDestInfo(cups::consts::http::CUPS_HTTP_DEFAULT, chosen_dest);

			// Set options
			let mut ptr_options = ptr::null_mut();
			let mut num_options = 0;
			num_options = cups::cupsAddOption(
				cups::consts::opts::CUPS_COPIES,
				c"1".as_ptr(),
				num_options,
				&mut ptr_options,
			);

			// Create job
			let mut job_id = 0;
			let status = cups::cupsCreateDestJob(
				cups::consts::http::CUPS_HTTP_DEFAULT,
				chosen_dest,
				info,
				&mut job_id,
				c"TestTitlePrintrs".as_ptr(), // FIXME
				num_options,
				ptr_options,
			);
			if status != cups::ipp_status_e::IPP_STATUS_OK {
				let message = cups::cupsLastErrorString();
				let message = ffi::CStr::from_ptr(message).to_string_lossy();
				eprintln!("Could not create print job: {message}");
				return;
			}
			eprintln!("Created job: {job_id}");

			// Initiate file transfer
			// FIXME: error handling
			let filename = path.file_name().expect("Could not extract file name");
			let filename =
				ffi::CString::new(filename.as_bytes()).expect("Could not create CString");

			let fstatus = cups::cupsStartDestDocument(
				cups::consts::http::CUPS_HTTP_DEFAULT,
				ptr_dests,
				info,
				job_id,
				filename.as_ptr(),
				cups::consts::format::CUPS_FORMAT_AUTO,
				num_options,
				ptr_options,
				cups::consts::bool::TRUE,
			);
			if fstatus != cups::http_status_e::HTTP_STATUS_CONTINUE {
				let message = cups::cupsLastErrorString();
				let message = ffi::CStr::from_ptr(message).to_string_lossy();
				eprintln!("Could not begin file transfer: {message}");
				return;
			}

			// Start file transfer
			let mut file = fs::File::open(path).expect("Could not open file");
			let mut buf = [0u8; 65536];
			loop {
				let length = file.read(&mut buf).expect("Could not read from file");
				if length == 0 {
					break;
				}

				let wstatus = cups::cupsWriteRequestData(
					cups::consts::http::CUPS_HTTP_DEFAULT,
					buf.as_ptr() as *const ffi::c_char,
					length,
				);
				if wstatus != cups::http_status_e::HTTP_STATUS_CONTINUE {
					let message = cups::cupsLastErrorString();
					let message = ffi::CStr::from_ptr(message).to_string_lossy();
					eprintln!("Could not transfer file bytes: {message}");
					return;
				}
			}

			// End file transfer
			let cstatus = cups::cupsFinishDestDocument(
				cups::consts::http::CUPS_HTTP_DEFAULT,
				ptr_dests,
				info,
			);
			if cstatus != cups::ipp_status_e::IPP_STATUS_OK {
				let message = cups::cupsLastErrorString();
				let message = ffi::CStr::from_ptr(message).to_string_lossy();
				eprintln!("Could not finish file transfer: {message}");
				return;
			}
			eprintln!("File transfer finished!");

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
