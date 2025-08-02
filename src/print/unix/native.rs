use std::collections::HashMap;
use std::ffi::CString;
use std::slice;

use crate::error::PrintError;
use crate::options::PrintOptions;
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

	fn get_printer(name: &str) -> Option<Printer> {
		let name = CString::new(name).ok()?;
		CupsDestination::new_by_name(name.as_c_str()).map(map_dest_to_printer)
	}

	fn get_default_printer() -> Option<Printer> {
		CupsDestination::new_default().map(map_dest_to_printer)
	}

	fn print<I, R>(readers: I, printer: &Printer, options: PrintOptions) -> Result<(), PrintError>
	where
		I: IntoIterator<Item = R>,
		R: std::io::Read,
	{
		let identifier = CString::new(printer.identifier.clone())?;
		let mut dest = CupsDestination::new_by_name(&identifier).ok_or(PrintError::NoPrinters)?;

		let cups_options = add_options(options, &mut dest)?;
		let context = job::PrintContext::new(dest, cups_options);

		let mut job = CupsJob::try_new("printrs", context)?;
		job.add_documents(readers)?;
		job.print()?;
		Ok(())
	}
}

fn add_options(
	options: PrintOptions,
	destination: &mut CupsDestination,
) -> Result<CupsOptions, PrintError> {
	// Maybe add a macro for this monstrosity?
	let mut cups_options = CupsOptions::new();
	add_option(options.copies, &mut cups_options, destination)?;
	add_option(options.finishings, &mut cups_options, destination)?;
	add_option(options.media_size, &mut cups_options, destination)?;
	add_option(options.media_source, &mut cups_options, destination)?;
	add_option(options.media_type, &mut cups_options, destination)?;
	add_option(options.number_up, &mut cups_options, destination)?;
	add_option(options.orientation, &mut cups_options, destination)?;
	add_option(options.color_mode, &mut cups_options, destination)?;
	add_option(options.quality, &mut cups_options, destination)?;
	add_option(options.sides_mode, &mut cups_options, destination)?;
	Ok(cups_options)
}

fn add_option<O: CupsOption>(
	option: Option<O>,
	cups_options: &mut CupsOptions,
	cups_destination: &mut CupsDestination,
) -> Result<(), PrintError> {
	let Some(option) = option else {
		return Ok(());
	};
	// validate:
	if !cups_options.validate(cups_destination, &option) {
		return Err(PrintError::UnsupportedOption {
			name: O::get_name().to_lowercase(),
			value: option.to_human_string(),
		});
	}
	// add:
	cups_options.add(&option);
	Ok(())
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
			identifier: cstr_to_string(dest.name),
			name: cstr_to_string(dest.name),
			instance,
			is_default: dest.is_default == cups::consts::bool(true),
			options,
		}
	}
}
