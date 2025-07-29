use std::collections::HashMap;
use std::io::Read;
use std::os::unix::ffi::OsStrExt;
use std::{borrow, ffi, fs, path, ptr, slice};

use crate::print::unix::cups;
use crate::print::{CrossPlatformApi, PlatformSpecificApi, Printer};

/// The size of the buffer that the file is read in chunks into.
const FILE_BUFFER_SIZE: usize = 65536; // 64 KiB

impl CrossPlatformApi for PlatformSpecificApi {
	fn get_printers() -> Vec<Printer> {
		unsafe {
			let mut ptr_dests = ptr::null_mut();
			let num_dests = cups::cupsGetDests(&mut ptr_dests);

			let dests = if num_dests > 0 {
				slice::from_raw_parts(ptr_dests, num_dests as usize)
			} else {
				cups::cupsFreeDests(num_dests, ptr_dests);
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

		// TODO: initializer for JobContext => guarantee JobContext is always safe
		// TODO: (i.e. pointers inside are valid) => no need to declare functions that take
		// TODO: &context as unsafe. Atm fns are marked safe but aren't
		let context = JobContext {
			http: cups::consts::http::CUPS_HTTP_DEFAULT,
			options: prepare_options_for_job(1),
			destination: chosen_dest,
			info: unsafe {
				cups::cupsCopyDestInfo(cups::consts::http::CUPS_HTTP_DEFAULT, chosen_dest)
			},
		};
		let job_id = create_job("printrs", &context);

		// Transfer file
		let file_name = path.file_name().expect("Could not extract file name"); // FIXME
		initiate_file_transfer(job_id, file_name, &context);
		transfer_file(path, &context);
		finish_file_transfer(&context);

		// FIXME: Free memory FIXME (choose dest differently?)
		unsafe {
			cups::cupsFreeDests(num_dests, ptr_dests);
		}
	}
}

/// Stores information related to a print job.
/// Implements [`Drop`] and will free the memory behind `options`
/// and `info` pointers once the owner goes out of scope.
struct JobContext {
	http: *mut cups::http_t,
	options: FatPointer<*mut cups::cups_option_t>,
	destination: *mut cups::cups_dest_t,
	info: *mut cups::cups_dinfo_t,
}
impl Drop for JobContext {
	fn drop(&mut self) {
		unsafe {
			cups::cupsFreeDestInfo(self.info);
			cups::cupsFreeOptions(self.options.num, self.options.ptr);
		}
	}
}

/// A pointer along with a size (useful for dynamic arrays).
#[derive(Clone, Copy)]
struct FatPointer<T> {
	num: ffi::c_int,
	ptr: T,
}

/// Configures options for the print job.
/// Returns a pointer to the options array.
fn prepare_options_for_job(copies: u32) -> FatPointer<*mut cups::cups_option_t> {
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

	FatPointer {
		num: num_options,
		ptr: ptr_options,
	}
}

/// Creates a print job.
fn create_job(title: &str, context: &JobContext) -> ffi::c_int {
	let title = ffi::CString::new(title).expect("Could not convert title to CString");
	let mut job_id = 0;

	unsafe {
		let status = cups::cupsCreateDestJob(
			context.http,
			context.destination,
			context.info,
			&mut job_id,
			title.as_ptr(),
			context.options.num,
			context.options.ptr,
		);
		if status != cups::ipp_status_e::IPP_STATUS_OK {
			let message = cups::cupsLastErrorString();
			eprintln!("Could not create print job: {}", cstr_to_str(message));
			panic!("djksl"); // FIXME
		}
	}

	eprintln!("Created job: {job_id}");
	job_id
}

/// Signals to initiate a file transfer to a specified print job.
fn initiate_file_transfer(job_id: ffi::c_int, file_name: &ffi::OsStr, context: &JobContext) {
	let filename = ffi::CString::new(file_name.as_bytes()).expect("Could not create CString"); // FIXME
	unsafe {
		let status = cups::cupsStartDestDocument(
			context.http,
			context.destination,
			context.info,
			job_id,
			filename.as_ptr(),
			cups::consts::format::CUPS_FORMAT_AUTO,
			context.options.num,
			context.options.ptr,
			cups::consts::bool::TRUE,
		);
		if status != cups::http_status_e::HTTP_STATUS_CONTINUE {
			let message = cups::cupsLastErrorString();
			eprintln!("Could not begin file transfer: {}", cstr_to_str(message));
			panic!("fjdksjrkekem"); // FIXME
		}
	}
}

/// Opens the file at the specified path, and transfers its contents.
fn transfer_file(path: &path::Path, context: &JobContext) {
	let mut file = fs::File::open(path).expect("Could not open file");
	let mut buf = [0u8; FILE_BUFFER_SIZE];

	loop {
		let length = file.read(&mut buf).expect("Could not read from file");
		if length == 0 {
			break;
		}
		unsafe {
			let status = cups::cupsWriteRequestData(
				context.http,
				buf.as_ptr() as *const ffi::c_char,
				length,
			);
			if status != cups::http_status_e::HTTP_STATUS_CONTINUE {
				let message = cups::cupsLastErrorString();
				eprintln!("Could not transfer file bytes: {}", cstr_to_str(message));
				return;
			}
		}
	}
}

/// Signals that the file transfer has finished.
fn finish_file_transfer(context: &JobContext) {
	unsafe {
		let status = cups::cupsFinishDestDocument(context.http, context.destination, context.info);
		if status != cups::ipp_status_e::IPP_STATUS_OK {
			let message = cups::cupsLastErrorString();
			eprintln!("Could not finish file transfer: {}", cstr_to_str(message));
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

/// Performs lossy conversion from a [`ffi::CStr`] into [`String`].
/// The result is either a borrowed value or an owned value.
unsafe fn cstr_to_str(ptr: *const ffi::c_char) -> borrow::Cow<'static, str> {
	unsafe { ffi::CStr::from_ptr(ptr).to_string_lossy() }
}
/// Constructs an owned UTF-8 string from a valid pointer to a valid C-string.
/// Invalid characters are replaced with the replacement character.
unsafe fn cstr_to_string(ptr: *const ffi::c_char) -> String {
	unsafe { cstr_to_str(ptr).into_owned() }
}
