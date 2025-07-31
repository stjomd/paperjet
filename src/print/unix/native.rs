use std::collections::HashMap;
use std::{path, ptr, slice};

use crate::error::PrintError;
use crate::print::unix::job::{CupsJob, FatPointerMut};
use crate::print::unix::{cstr_to_string, cups, job};
use crate::print::{CrossPlatformApi, PlatformSpecificApi, Printer};

/// A struct representing an array of CUPS destinations.
pub struct CupsDestinations {
	/// A fat pointer to the array of destinations allocated by CUPS.
	dests: FatPointerMut<cups::cups_dest_t>,
}
impl CupsDestinations {
	/// Creates a new instance of this struct, retrieving CUPS destinations.
	pub fn new() -> Self {
		let mut dests_ptr = ptr::null_mut();
		// SAFETY: cupsGetDests accepts any pointer to a pointer, and will overwrite `dests_ptr` with
		// the pointer to the allocated array.
		let dests_num = unsafe { cups::cupsGetDests(&mut dests_ptr) };
		Self {
			dests: FatPointerMut {
				num: dests_num,
				ptr: dests_ptr,
			},
		}
	}
	/// Returns a view into the CUPS destinations as a slice.
	fn as_slice_mut(&mut self) -> &mut [cups::cups_dest_t] {
		if self.dests.num > 0 {
			// SAFETY: `self.dests` contains a valid pointer & size created in the `Self::new` function.
			unsafe { slice::from_raw_parts_mut(self.dests.ptr, self.dests.num as usize) }
		} else {
			&mut []
		}
	}
	/// Returns a destination at the specified index, or [`None`] if the index is invalid.
	pub fn get_mut(&mut self, index: usize) -> Option<&mut cups::cups_dest_t> {
		self.as_slice_mut().get_mut(index)
	}
}
impl Drop for CupsDestinations {
	fn drop(&mut self) {
		// SAFETY: `self.dests` contains a valid pointer & size created in the `Self::new` function.
		unsafe { cups::cupsFreeDests(self.dests.num, self.dests.ptr) };
	}
}

impl CrossPlatformApi for PlatformSpecificApi {
	fn get_printers() -> Vec<Printer> {
		CupsDestinations::new()
			.as_slice_mut()
			.iter()
			.map(map_dest_to_printer)
			.collect()
	}

	fn print_file(path: &path::Path) -> Result<(), PrintError> {
		let mut dests = CupsDestinations::new();
		let chosen_dest = dests.get_mut(0).ok_or(PrintError::NoPrinters)?;

		let context = job::PrintContext {
			http: cups::consts::http::CUPS_HTTP_DEFAULT,
			options: job::prepare_options_for_job(1)?,
			destination: chosen_dest,
			info: unsafe {
				cups::cupsCopyDestInfo(cups::consts::http::CUPS_HTTP_DEFAULT, chosen_dest)
			},
		};

		let job = CupsJob::try_new("printrs", context)?;
		job.add_documents([path])?;
		job.print()?;
		Ok(())
	}
}

/// Maps an instance of [`cups::cups_dest_t`] to a [`Printer`].
/// The argument's pointers must all be valid.
fn map_dest_to_printer(dest: &cups::cups_dest_t) -> Printer {
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
