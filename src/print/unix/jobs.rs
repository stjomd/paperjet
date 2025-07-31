use crate::print::unix::{cstr_to_str, cups};
use std::io::BufRead;
use std::os::unix::ffi::OsStrExt;
use std::{ffi, fs, io, path, ptr};

/// The size of the buffer that the file is read in chunks into.
const FILE_BUFFER_SIZE: usize = 65536; // 64 KiB

/// Stores information related to a print job.
/// Implements [`Drop`] and will free the memory behind `options`
/// and `info` pointers once the owner goes out of scope.
pub struct PrintContext {
	pub http: *mut cups::http_t,
	pub options: FatPointerMut<cups::cups_option_t>,
	pub destination: *mut cups::cups_dest_t,
	pub info: *mut cups::cups_dinfo_t,
}
impl Drop for PrintContext {
	fn drop(&mut self) {
		unsafe {
			cups::cupsFreeDestInfo(self.info);
			cups::cupsFreeOptions(self.options.num, self.options.ptr);
		}
	}
}

/// A mutable pointer along with a size (useful for dynamic arrays).
#[derive(Clone, Copy)]
pub struct FatPointerMut<T> {
	num: ffi::c_int,
	ptr: *mut T,
}

/// Configures options for the print job.
/// Returns a pointer to the options array.
pub fn prepare_options_for_job(copies: u32) -> FatPointerMut<cups::cups_option_t> {
	let mut ptr_options = ptr::null_mut();
	let mut num_options = 0;

	let copies = copies.to_string();
	let copies = ffi::CString::new(copies).expect("Could not convert copies to CString"); // FIXME
	num_options = unsafe {
		cups::cupsAddOption(
			cups::consts::opts::CUPS_COPIES.as_ptr(),
			copies.as_ptr(),
			num_options,
			&mut ptr_options,
		)
	};

	FatPointerMut {
		num: num_options,
		ptr: ptr_options,
	}
}

/// Creates a print job.
pub fn create_job(title: &str, context: &PrintContext) -> ffi::c_int {
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
pub fn initiate_file_transfer(job_id: ffi::c_int, file_name: &ffi::OsStr, context: &PrintContext) {
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
pub fn transfer_file(path: &path::Path, context: &PrintContext) {
	let file = fs::File::open(path).expect("Could not open file");
	let mut reader = io::BufReader::with_capacity(FILE_BUFFER_SIZE, file);

	loop {
		let buf = reader.fill_buf().expect("Could not read from file");
		let buf_len = buf.len();

		if buf_len == 0 {
			break;
		}
		unsafe {
			let status = cups::cupsWriteRequestData(
				context.http,
				buf.as_ptr() as *const ffi::c_char,
				buf_len,
			);
			if status != cups::http_status_e::HTTP_STATUS_CONTINUE {
				let message = cups::cupsLastErrorString();
				eprintln!("Could not transfer file bytes: {}", cstr_to_str(message));
				return;
			}
		}

		reader.consume(buf_len);
	}
}

/// Signals that the file transfer has finished.
pub fn finish_file_transfer(context: &PrintContext) {
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
