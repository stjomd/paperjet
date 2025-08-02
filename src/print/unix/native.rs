use std::collections::HashMap;
use std::slice;

use crate::error::PrintError;
use crate::options::CopiesInt;
use crate::print::unix::dest::{CupsDestination, CupsDestinations};
use crate::print::unix::job::CupsJob;
use crate::print::unix::options::{CupsOption, CupsOptions};
use crate::print::unix::{cstr_to_string, cups, job};
use crate::print::{CrossPlatformApi, PlatformSpecificApi, Printer};

impl CrossPlatformApi for PlatformSpecificApi {
	fn get_printers() -> Vec<Printer> {
		CupsDestinations::new()
			.into_iter()
			.map(map_dest_to_printer)
			.collect()
	}
	fn print<I, R>(readers: I) -> Result<(), PrintError>
	where
		I: IntoIterator<Item = R>,
		R: std::io::Read,
	{
		let mut dests = CupsDestinations::new();
		let mut chosen_dest = dests.get(0).ok_or(PrintError::NoPrinters)?;

		let mut options = CupsOptions::new();
		let opt = CopiesInt(1);
		let valid = options.validate(&mut chosen_dest, &opt);
		if !valid {
			return Err(PrintError::UnsupportedOption {
				name: "CopiesInt".to_owned(),
				value: opt.get_cups_option_value().to_string_lossy().into_owned(),
			});
		}
		options.add(&opt);

		let context = job::PrintContext::new(chosen_dest, options);

		let mut job = CupsJob::try_new("printrs", context)?;
		job.add_documents(readers)?;
		job.print()?;
		Ok(())
	}
}

/// Maps an instance of [`cups::cups_dest_t`] to a [`Printer`].
/// The argument's pointers must all be valid.
fn map_dest_to_printer(dest: CupsDestination) -> Printer {
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
