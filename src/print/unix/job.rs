use crate::error::PrintError;
use crate::print::unix::{cstr_to_string, cups};
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
	pub num: ffi::c_int,
	pub ptr: *mut T,
}

/// Configures options for the print job.
/// Returns a pointer to the options array.
pub fn prepare_options_for_job(
	copies: u32,
) -> Result<FatPointerMut<cups::cups_option_t>, PrintError> {
	let mut ptr_options = ptr::null_mut();
	let mut num_options = 0;

	let copies = copies.to_string();
	let copies = ffi::CString::new(copies)?;
	num_options = unsafe {
		cups::cupsAddOption(
			cups::consts::opts::CUPS_COPIES.as_ptr(),
			copies.as_ptr(),
			num_options,
			&mut ptr_options,
		)
	};

	Ok(FatPointerMut {
		num: num_options,
		ptr: ptr_options,
	})
}

#[allow(dead_code)]
/// A struct that represents a CUPS job.
pub struct CupsJob {
	/// The job ID, assigned by CUPS.
	id: ffi::c_int,
	/// The context of the print job.
	context: PrintContext,
	/// Flag indicating whether the job should be cancelled when the value is dropped.
	cancel_on_drop: bool,
}
impl CupsJob {
	/// Creates a CUPS job.
	/// If successful, this will result in a new job on the CUPS server.
	pub fn try_new(title: &str, context: PrintContext) -> Result<Self, PrintError> {
		let job_id = create_job(title, &context)?;
		Ok(Self {
			id: job_id,
			context,
			cancel_on_drop: true,
		})
	}
	/// Adds documents, specified by their paths, to this job.
	pub fn add_documents<'a, I>(&self, docs: I) -> Result<(), PrintError>
	where
		I: IntoIterator<Item = &'a path::Path>,
	{
		for doc in docs {
			self.add_document(doc)?;
		}
		Ok(())
	}
	/// Adds a document with the specified path to this job.
	fn add_document(&self, path: &path::Path) -> Result<(), PrintError> {
		let file_name = path
			.file_name()
			.ok_or_else(|| PrintError::InvalidPath(path.to_owned()))?;
		initiate_file_transfer(self.id, file_name, &self.context)?;
		transfer_file(path, &self.context)?;
		finish_file_transfer(&self.context)?;
		Ok(())
	}
	/// Closes this job and starts printing.
	pub fn print(mut self) -> Result<(), PrintError> {
		close_job(self.id, &self.context)?;
		self.cancel_on_drop = false;
		Ok(())
	}
}
impl Drop for CupsJob {
	fn drop(&mut self) {
		if self.cancel_on_drop {
			let _ = cancel_job(self.id, &self.context)
				.inspect_err(|e| eprintln!("could not cancel job during drop: {e}"));
		}
	}
}

/// Creates a print job.
fn create_job(title: &str, context: &PrintContext) -> Result<ffi::c_int, PrintError> {
	let title = ffi::CString::new(title)?;
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
			return Err(get_last_error());
		}
	}

	Ok(job_id)
}

/// Signals to initiate a file transfer to a specified print job.
fn initiate_file_transfer(
	job_id: ffi::c_int,
	file_name: &ffi::OsStr,
	context: &PrintContext,
) -> Result<(), PrintError> {
	let filename = ffi::CString::new(file_name.as_bytes())?;
	unsafe {
		let status = cups::cupsStartDestDocument(
			context.http,
			context.destination,
			context.info,
			job_id,
			filename.as_ptr(),
			cups::consts::format::CUPS_FORMAT_AUTO.as_ptr(),
			context.options.num,
			context.options.ptr,
			cups::consts::bool(false), // we always pass `false` here & start printing with closeDestJob
		);
		if status != cups::http_status_e::HTTP_STATUS_CONTINUE {
			return Err(get_last_error());
		}
	}
	Ok(())
}

/// Opens the file at the specified path, and transfers its contents.
fn transfer_file(path: &path::Path, context: &PrintContext) -> Result<(), PrintError> {
	let file = fs::File::open(path)?;
	let mut reader = io::BufReader::with_capacity(FILE_BUFFER_SIZE, file);

	loop {
		let buf = reader.fill_buf()?;
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
				return Err(get_last_error());
			}
		}
		reader.consume(buf_len);
	}

	Ok(())
}

/// Signals that the file transfer has finished.
fn finish_file_transfer(context: &PrintContext) -> Result<(), PrintError> {
	unsafe {
		let status = cups::cupsFinishDestDocument(context.http, context.destination, context.info);
		if status != cups::ipp_status_e::IPP_STATUS_OK {
			return Err(get_last_error());
		}
	}
	Ok(())
}

/// Cancels the job with the specified ID.
fn cancel_job(job_id: ffi::c_int, context: &PrintContext) -> Result<(), PrintError> {
	let status = unsafe { cups::cupsCancelDestJob(context.http, context.destination, job_id) };
	if status != cups::ipp_status_e::IPP_STATUS_OK {
		return Err(get_last_error());
	}
	Ok(())
}

/// Closes the job with the specified ID and starts printing.
fn close_job(job_id: ffi::c_int, context: &PrintContext) -> Result<(), PrintError> {
	let status =
		unsafe { cups::cupsCloseDestJob(context.http, context.destination, context.info, job_id) };
	if status != cups::ipp_status_e::IPP_STATUS_OK {
		return Err(get_last_error());
	}
	Ok(())
}

/// Retrieves the last error string from CUPS and constructs a [`PrintError::Backend`].
/// If no error string is returned by CUPS, an empty error string is used.
fn get_last_error() -> PrintError {
	let message = unsafe {
		let ptr = cups::cupsLastErrorString();
		if !ptr.is_null() {
			cstr_to_string(ptr)
		} else {
			String::from("")
		}
	};
	PrintError::Backend(message)
}
