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
		let mut ptr_dests = ptr::null_mut();
		let num_dests = unsafe { cups::cupsGetDests(&mut ptr_dests) };
		let chosen_dest = ptr_dests; // first FIXME

		unsafe {
			// Set up job
			let info = cups::cupsCopyDestInfo(cups::consts::http::CUPS_HTTP_DEFAULT, chosen_dest);
			let options = prepare_options_for_job(1);
			let job_id = create_job("printrs", options, chosen_dest, info);

			// Transfer file
			let file_name = path.file_name().expect("Could not extract file name"); // FIXME
			initiate_file_transfer(job_id, file_name, chosen_dest, info, options);
			transfer_file(path);
			finish_file_transfer(chosen_dest, info);

			// Free memory
			cups::cupsFreeDestInfo(info);
			cups::cupsFreeOptions(options.num, options.ptr);
			cups::cupsFreeDests(num_dests, ptr_dests);
		}
	}
}

/// A pointer along with a size (useful for dynamic arrays).
#[derive(Clone, Copy)]
struct FatPointer<T> {
	num: ffi::c_int,
	ptr: T,
}
type OptionsPointer = FatPointer<*mut cups::cups_option_t>;

/// Configures options for the print job.
/// Returns a pointer to the options array.
fn prepare_options_for_job(copies: u32) -> OptionsPointer {
	let mut ptr_options = ptr::null_mut();
	let mut num_options = 0;

	let copies = copies.to_string();
	let copies = ffi::CString::new(copies).expect("Could not convert copies to CString"); // FIXME
	num_options = unsafe {
		cups::cupsAddOption(
			cups::consts::opts::CUPS_COPIES,
			copies.as_ptr(),
			num_options,
			&mut ptr_options,
		)
	};

	OptionsPointer {
		num: num_options,
		ptr: ptr_options,
	}
}

/// Creates a print job.
unsafe fn create_job(
	title: &str,
	options: OptionsPointer,
	dest: *mut cups::cups_dest_t,
	info: *mut cups::cups_dinfo_t,
) -> ffi::c_int {
	let title = ffi::CString::new(title).expect("Could not convert title to CString");
	let mut job_id = 0;

	unsafe {
		let status = cups::cupsCreateDestJob(
			cups::consts::http::CUPS_HTTP_DEFAULT,
			dest,
			info,
			&mut job_id,
			title.as_ptr(),
			options.num,
			options.ptr,
		);
		if status != cups::ipp_status_e::IPP_STATUS_OK {
			let message = cups::cupsLastErrorString();
			let message = ffi::CStr::from_ptr(message).to_string_lossy();
			eprintln!("Could not create print job: {message}");
			panic!("djksl"); // FIXME
		}
	}

	eprintln!("Created job: {job_id}");
	job_id
}

/// Signals to initiate a file transfer to a specified print job.
unsafe fn initiate_file_transfer(
	job_id: ffi::c_int,
	file_name: &ffi::OsStr,
	dest: *mut cups::cups_dest_t,
	info: *mut cups::cups_dinfo_t,
	options: OptionsPointer,
) {
	let filename = ffi::CString::new(file_name.as_bytes()).expect("Could not create CString"); // FIXME
	unsafe {
		let fstatus = cups::cupsStartDestDocument(
			cups::consts::http::CUPS_HTTP_DEFAULT,
			dest,
			info,
			job_id,
			filename.as_ptr(),
			cups::consts::format::CUPS_FORMAT_AUTO,
			options.num,
			options.ptr,
			cups::consts::bool::TRUE,
		);
		if fstatus != cups::http_status_e::HTTP_STATUS_CONTINUE {
			let message = cups::cupsLastErrorString();
			let message = ffi::CStr::from_ptr(message).to_string_lossy();
			eprintln!("Could not begin file transfer: {message}");
			panic!("fjdksjrkekem"); // FIXME
		}
	}
}

/// Opens the file at the specified path, and transfers its contents.
fn transfer_file(path: &path::Path) {
	let mut file = fs::File::open(path).expect("Could not open file");
	let mut buf = [0u8; 65536];

	loop {
		let length = file.read(&mut buf).expect("Could not read from file");
		if length == 0 {
			break;
		}
		unsafe {
			let status = cups::cupsWriteRequestData(
				cups::consts::http::CUPS_HTTP_DEFAULT,
				buf.as_ptr() as *const ffi::c_char,
				length,
			);
			if status != cups::http_status_e::HTTP_STATUS_CONTINUE {
				let message = cups::cupsLastErrorString();
				let message = ffi::CStr::from_ptr(message).to_string_lossy();
				eprintln!("Could not transfer file bytes: {message}");
				return;
			}
		}
	}
}

/// Signals that the file transfer has finished.
unsafe fn finish_file_transfer(dest: *mut cups::cups_dest_t, info: *mut cups::cups_dinfo_t) {
	unsafe {
		let status =
			cups::cupsFinishDestDocument(cups::consts::http::CUPS_HTTP_DEFAULT, dest, info);
		if status != cups::ipp_status_e::IPP_STATUS_OK {
			let message = cups::cupsLastErrorString();
			let message = ffi::CStr::from_ptr(message).to_string_lossy();
			eprintln!("Could not finish file transfer: {message}");
			panic!("mkqkiswjui"); // FIXMEs
		}
	}
	eprintln!("File transfer finished!");
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
