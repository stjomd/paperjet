use std::collections::HashMap;
use std::{path, ptr, slice};

use crate::error::PrintError;
use crate::print::unix::job::CupsJob;
use crate::print::unix::{cstr_to_string, cups, job};
use crate::print::{CrossPlatformApi, PlatformSpecificApi, Printer};

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

	fn print_file(path: &path::Path) -> Result<(), PrintError> {
		let mut ptr_dests = ptr::null_mut();
		let _ = unsafe { cups::cupsGetDests(&mut ptr_dests) };
		let chosen_dest = ptr_dests; // first FIXME

		// TODO: initializer for JobContext => guarantee JobContext is always safe
		// TODO: (i.e. pointers inside are valid) => no need to declare functions that take
		// TODO: &context as unsafe. Atm fns are marked safe but aren't
		let context = job::PrintContext {
			http: cups::consts::http::CUPS_HTTP_DEFAULT,
			options: job::prepare_options_for_job(1)?,
			destination: chosen_dest,
			info: unsafe {
				cups::cupsCopyDestInfo(cups::consts::http::CUPS_HTTP_DEFAULT, chosen_dest)
			},
		};
		// TODO: CupsJob contains destination (cups_dest_t). Be careful not to free ptr_dests before
		// dropping CupsJob - or cancelling the job won't work.

		let job = CupsJob::try_new("printrs", context)?;
		job.add_documents([path])?;
		job.print()?;
		Ok(())
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
