use std::collections::HashMap;
use std::{path, slice};

use crate::error::PrintError;
use crate::print::unix::dest::CupsDestinations;
use crate::print::unix::job::CupsJob;
use crate::print::unix::{cstr_to_string, cups, job};
use crate::print::{CrossPlatformApi, PlatformSpecificApi, Printer};

impl CrossPlatformApi for PlatformSpecificApi {
	fn get_printers() -> Vec<Printer> {
		CupsDestinations::new()
			.as_slice()
			.iter()
			.map(map_dest_to_printer)
			.collect()
	}

	fn print_files(paths: &[path::PathBuf]) -> Result<(), PrintError> {
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
		job.add_documents(paths)?;
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
			is_default: dest.is_default == cups::consts::bool(true),
			options,
		}
	}
}
