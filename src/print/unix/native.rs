use std::collections::HashMap;
use std::ffi::CString;
use std::slice;

use crate::error::PrintError;
use crate::options::PrintOptions;
use crate::print::unix::cups;
use crate::print::unix::dest::{CupsDestination, CupsDestinationInfo, CupsDestinations};
use crate::print::unix::job::CupsJob;
use crate::print::unix::options::{CupsOption, CupsOptions};
use crate::print::{CrossPlatformApi, PlatformSpecificApi, Printer, util};

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

	fn print<I, R>(readers: I, printer: Printer, options: PrintOptions) -> Result<(), PrintError>
	where
		I: IntoIterator<Item = R>,
		R: std::io::Read,
	{
		let id = CString::new(printer.identifier.clone())?;
		let mut cups_dest = CupsDestination::new_by_name(&id)
			.ok_or(PrintError::PrinterNotFound(printer.identifier))?;

		let mut cups_info = CupsDestinationInfo::new(&mut cups_dest).ok_or(
			PrintError::NecessaryInformationMissing(String::from("no CUPS destination info")),
		)?;
		let cups_opts = add_options(options, &mut cups_dest, &mut cups_info)?;
		let mut cups_job = CupsJob::try_new("printrs", cups_dest, cups_info, cups_opts)?;

		cups_job.add_documents(readers)?;
		cups_job.print()
	}
}

fn add_options(
	options: PrintOptions,
	destination: &mut CupsDestination,
	info: &mut CupsDestinationInfo,
) -> Result<CupsOptions, PrintError> {
	// Maybe add a macro for this monstrosity?
	let mut cups_options = CupsOptions::new();
	add_option(options.copies, &mut cups_options, destination, info)?;
	add_option(options.finishings, &mut cups_options, destination, info)?;
	add_option(options.media_size, &mut cups_options, destination, info)?;
	add_option(options.media_source, &mut cups_options, destination, info)?;
	add_option(options.media_type, &mut cups_options, destination, info)?;
	add_option(options.number_up, &mut cups_options, destination, info)?;
	add_option(options.orientation, &mut cups_options, destination, info)?;
	add_option(options.color_mode, &mut cups_options, destination, info)?;
	add_option(options.quality, &mut cups_options, destination, info)?;
	add_option(options.sides_mode, &mut cups_options, destination, info)?;
	Ok(cups_options)
}

fn add_option<O: CupsOption>(
	option: Option<O>,
	cups_options: &mut CupsOptions,
	cups_destination: &mut CupsDestination,
	cups_info: &mut CupsDestinationInfo,
) -> Result<(), PrintError> {
	let Some(option) = option else {
		return Ok(());
	};
	// validate:
	if !cups_options.validate(cups_destination, cups_info, &option) {
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
			.map(|opt| {
				(
					util::cstr_to_string(opt.name),
					util::cstr_to_string(opt.value),
				)
			})
			.collect::<HashMap<String, String>>();

		let instance = if !dest.instance.is_null() {
			Some(util::cstr_to_string(dest.instance))
		} else {
			None
		};

		Printer {
			identifier: util::cstr_to_string(dest.name),
			name: util::cstr_to_string(dest.name),
			instance,
			is_default: dest.is_default == cups::consts::bool(true),
			options,
		}
	}
}
