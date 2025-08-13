use std::borrow::Cow;
use std::ffi::{CStr, CString};
use std::ptr;

use crate::options::*;
use crate::print::unix::cups;
use crate::print::unix::cups::consts::opts;
use crate::print::unix::dest::{CupsDestination, CupsDestinationInfo};
use crate::print::util::FatPointerMut;

// MARK: - Cups Options Struct

/// A struct representing an array of options allocated by CUPS.
#[derive(Debug)]
pub struct CupsOptions {
	/// A fat pointer to the array of options.
	opts: FatPointerMut<cups::cups_option_t>,
}
impl CupsOptions {
	/// Creates a new, empty list of CUPS options.
	#[allow(clippy::new_without_default)]
	pub fn new() -> Self {
		Self {
			opts: FatPointerMut {
				size: 0,
				ptr: ptr::null_mut(),
			},
		}
	}
	/// Adds a CUPS option to this instance.
	pub fn add<O>(&mut self, option: &O)
	where
		O: CupsOption,
	{
		// SAFETY: `cupsAddOption` accepts a name, value, current number of elements, and a pointer to
		// `*mut cups_option_t`. It returns the new number of elements and writes a valid pointer
		// into `self.opts.ptr`, after allocating an array of options. Thus repeated calls are safe
		// until the memory is freed by calling `cupsFreeOptions`.
		unsafe {
			self.opts.size = cups::cupsAddOption(
				O::get_cups_option_name().as_ptr(),
				option.get_cups_option_value().as_ptr(),
				self.opts.size,
				&mut self.opts.ptr,
			);
		};
	}
	/// Checks with a particular destination whether the option and its value are supported.
	pub fn validate<O>(
		&self,
		destination: &mut CupsDestination,
		info: &mut CupsDestinationInfo,
		option: &O,
	) -> bool
	where
		O: CupsOption,
	{
		// SAFETY: `destination` is a CupsDestination instance, which can only be constructed safely,
		// contains a reference and thus the pointer it dereferences to is valid.
		let result = unsafe {
			cups::cupsCheckDestSupported(
				cups::consts::http::CUPS_HTTP_DEFAULT,
				destination.as_mut_ptr(),
				info.as_mut_ptr(),
				O::get_cups_option_name().as_ptr(),
				option.get_cups_option_value().as_ptr(),
			)
		};
		result == cups::consts::bool(true)
	}

	/// Converts this options list into a fat pointer, containing a pointer to the first element,
	/// as well as a valid size.
	///
	/// **DANGER**: If this options list is empty, a null pointer is returned.
	/// However, this is fine if passed to CUPS.
	pub fn as_fat_ptr_mut(&mut self) -> FatPointerMut<cups::cups_option_t> {
		self.opts
	}
}
impl Drop for CupsOptions {
	fn drop(&mut self) {
		if self.opts.ptr.is_null() {
			return;
		}
		// SAFETY: `self.opts` is a valid, non-null pointer, pointing to memory allocated by CUPS.
		// It remains valid until `cupsFreeOptions` is called, which is now.
		unsafe { cups::cupsFreeOptions(self.opts.size, self.opts.ptr) };
	}
}

// MARK: - CupsOption trait

/// A trait that designates an option that can be converted to a CUPS option value string.
pub trait CupsOption: PrintOption {
	/// Converts the option's type to a name accepted by CUPS.
	/// Returns a borrowed C string.
	fn get_cups_option_name() -> &'static CStr;
	/// Converts the option's value to a string accepted by CUPS.
	/// Returns either a borrowed or an owned value inside a [`Cow`] pointer.
	fn get_cups_option_value(&self) -> Cow<'static, CStr>;
}

impl CupsOption for CopiesInt {
	fn get_cups_option_name() -> &'static CStr {
		opts::CUPS_COPIES
	}
	fn get_cups_option_value(&self) -> Cow<'static, CStr> {
		let string = self.0.to_string();
		// SAFETY: `string` is built from `self.0`, which is a C integer, and thus contains only bytes
		// which correspond to digit characters, and no 0 bytes.
		let c_string = CString::new(string).expect("Could not convert copies to CString");
		Cow::Owned(c_string)
	}
}

impl CupsOption for Finishing {
	fn get_cups_option_name() -> &'static CStr {
		opts::CUPS_FINISHINGS
	}
	fn get_cups_option_value(&self) -> Cow<'static, CStr> {
		Cow::Borrowed(match self {
			Finishing::Bind => opts::values::CUPS_FINISHINGS_BIND,
			Finishing::Cover => opts::values::CUPS_FINISHINGS_COVER,
			Finishing::Fold => opts::values::CUPS_FINISHINGS_FOLD,
			Finishing::Punch => opts::values::CUPS_FINISHINGS_PUNCH,
			Finishing::Staple => opts::values::CUPS_FINISHINGS_STAPLE,
			Finishing::Trim => opts::values::CUPS_FINISHINGS_TRIM,
		})
	}
}
impl CupsOption for Vec<Finishing> {
	fn get_cups_option_name() -> &'static CStr {
		opts::CUPS_FINISHINGS
	}
	fn get_cups_option_value(&self) -> Cow<'static, CStr> {
		if self.is_empty() {
			return Cow::Borrowed(opts::values::CUPS_FINISHINGS_NONE);
		}
		// We want a comma-separated string here:
		let bytes = self
			.iter()
			.map(|finishing| finishing.get_cups_option_value())
			.map(|cow| cow.to_bytes().to_vec())
			.collect::<Vec<Vec<u8>>>()
			.join(b",".as_slice());

		// SAFETY: `bytes` are constructed from valid C strings and the ',' byte,
		// and thus do not contain 0 bytes.
		let c_string = CString::new(bytes)
			.expect("Could not convert comma-separated string of finishing options to CString");
		Cow::Owned(c_string)
	}
}

impl CupsOption for MediaSize {
	fn get_cups_option_name() -> &'static CStr {
		opts::CUPS_MEDIA
	}
	fn get_cups_option_value(&self) -> Cow<'static, CStr> {
		Cow::Borrowed(match self {
			// ISO & A3+
			MediaSize::A3 => opts::values::CUPS_MEDIA_A3,
			MediaSize::A3Plus => opts::values::CUPS_MEDIA_SUPERBA3,
			MediaSize::A4 => opts::values::CUPS_MEDIA_A4,
			MediaSize::A5 => opts::values::CUPS_MEDIA_A5,
			MediaSize::A6 => opts::values::CUPS_MEDIA_A6,
			// US
			MediaSize::GovtLetter => opts::values::CUPS_MEDIA_8X10,
			MediaSize::Letter => opts::values::CUPS_MEDIA_LETTER,
			MediaSize::Legal => opts::values::CUPS_MEDIA_LEGAL,
			MediaSize::Tabloid => opts::values::CUPS_MEDIA_TABLOID,
			// Miscellaneous
			MediaSize::Index3x5 => opts::values::CUPS_MEDIA_3X5,
			MediaSize::Index4x6 => opts::values::CUPS_MEDIA_4X6,
			MediaSize::Index5x7 => opts::values::CUPS_MEDIA_5X7,
			MediaSize::Envelope10 => opts::values::CUPS_MEDIA_ENV10,
			MediaSize::EnvelopeDL => opts::values::CUPS_MEDIA_ENVDL,
			MediaSize::Photo3R => opts::values::CUPS_MEDIA_PHOTO_L,
		})
	}
}

impl CupsOption for MediaSource {
	fn get_cups_option_name() -> &'static CStr {
		opts::CUPS_MEDIA_SOURCE
	}
	fn get_cups_option_value(&self) -> Cow<'static, CStr> {
		Cow::Borrowed(match self {
			MediaSource::Auto => opts::values::CUPS_MEDIA_SOURCE_AUTO,
			MediaSource::Manual => opts::values::CUPS_MEDIA_SOURCE_MANUAL,
		})
	}
}

impl CupsOption for MediaType {
	fn get_cups_option_name() -> &'static CStr {
		opts::CUPS_MEDIA_TYPE
	}
	fn get_cups_option_value(&self) -> Cow<'static, CStr> {
		Cow::Borrowed(match self {
			MediaType::Auto => opts::values::CUPS_MEDIA_TYPE_AUTO,
			MediaType::Envelope => opts::values::CUPS_MEDIA_TYPE_ENVELOPE,
			MediaType::Labels => opts::values::CUPS_MEDIA_TYPE_LABELS,
			MediaType::Letterhead => opts::values::CUPS_MEDIA_TYPE_LETTERHEAD,
			MediaType::Photo => opts::values::CUPS_MEDIA_TYPE_PHOTO,
			MediaType::PhotoGlossy => opts::values::CUPS_MEDIA_TYPE_PHOTO_GLOSSY,
			MediaType::PhotoMatte => opts::values::CUPS_MEDIA_TYPE_PHOTO_MATTE,
			MediaType::Plain => opts::values::CUPS_MEDIA_TYPE_PLAIN,
			MediaType::Transparent => opts::values::CUPS_MEDIA_TYPE_TRANSPARENCY,
		})
	}
}

impl CupsOption for NumberUpInt {
	fn get_cups_option_name() -> &'static CStr {
		opts::CUPS_NUMBER_UP
	}
	fn get_cups_option_value(&self) -> Cow<'static, CStr> {
		let string = self.0.to_string();
		// SAFETY: `string` is built from `self.0`, which is a C integer, and thus contains only bytes
		// which correspond to digit characters, and no 0 bytes.
		let c_string = CString::new(string).expect("Could not convert number up to CString");
		Cow::Owned(c_string)
	}
}

impl CupsOption for Orientation {
	fn get_cups_option_name() -> &'static CStr {
		opts::CUPS_ORIENTATION
	}
	fn get_cups_option_value(&self) -> Cow<'static, CStr> {
		Cow::Borrowed(match self {
			Orientation::Portrait => opts::values::CUPS_ORIENTATION_PORTRAIT,
			Orientation::Landscape => opts::values::CUPS_ORIENTATION_LANDSCAPE,
		})
	}
}

impl CupsOption for ColorMode {
	fn get_cups_option_name() -> &'static CStr {
		opts::CUPS_PRINT_COLOR_MODE
	}
	fn get_cups_option_value(&self) -> Cow<'static, CStr> {
		Cow::Borrowed(match self {
			ColorMode::Auto => opts::values::CUPS_PRINT_COLOR_MODE_AUTO,
			ColorMode::Monochrome => opts::values::CUPS_PRINT_COLOR_MODE_MONOCHROME,
			ColorMode::Color => opts::values::CUPS_PRINT_COLOR_MODE_COLOR,
		})
	}
}

impl CupsOption for Quality {
	fn get_cups_option_name() -> &'static CStr {
		opts::CUPS_PRINT_QUALITY
	}
	fn get_cups_option_value(&self) -> Cow<'static, CStr> {
		Cow::Borrowed(match self {
			Quality::Draft => opts::values::CUPS_PRINT_QUALITY_DRAFT,
			Quality::Normal => opts::values::CUPS_PRINT_QUALITY_NORMAL,
			Quality::High => opts::values::CUPS_PRINT_QUALITY_HIGH,
		})
	}
}

impl CupsOption for SidesMode {
	fn get_cups_option_name() -> &'static CStr {
		opts::CUPS_SIDES
	}
	fn get_cups_option_value(&self) -> Cow<'static, CStr> {
		Cow::Borrowed(match self {
			SidesMode::OneSided => opts::values::CUPS_SIDES_ONE_SIDED,
			SidesMode::TwoSidedPortrait => opts::values::CUPS_SIDES_TWO_SIDED_PORTRAIT,
			SidesMode::TwoSidedLandscape => opts::values::CUPS_SIDES_TWO_SIDED_LANDSCAPE,
		})
	}
}

#[cfg(test)]
mod tests {
	use std::ffi::CString;
	use std::ops::Deref;

	use crate::options::Finishing;
	use crate::print::unix::cups::consts::opts;
	use crate::print::unix::options::CupsOption;

	#[test]
	fn if_empty_finishings_then_cups_finishings_none() {
		// Finishings are empty:
		let finishings: Vec<Finishing> = Vec::new();

		// The CUPS option value should be CUPS_FINISHINGS_NONE:
		let value = finishings.get_cups_option_value();

		assert_eq!(
			opts::values::CUPS_FINISHINGS_NONE,
			value.deref(),
			// message:
			"Empty finishings should have value '{}', was: '{}'",
			opts::values::CUPS_FINISHINGS_NONE
				.to_str()
				.expect("Can't convert CUPS const to String"),
			value
				.to_str()
				.expect("Can't convert CUPS option value to String")
		)
	}

	#[test]
	fn if_one_finishing_then_cups_finishing_constant() {
		// Only one finishing is present:
		let finishings = vec![Finishing::Staple];

		// The CUPS option value should be CUPS_FINISHINGS_STAPLE:
		let value = finishings.get_cups_option_value();

		assert_eq!(
			opts::values::CUPS_FINISHINGS_STAPLE,
			value.deref(),
			// message:
			"Finishings should have value '{}', was: '{}'",
			opts::values::CUPS_FINISHINGS_STAPLE
				.to_str()
				.expect("Can't convert CUPS const to String"),
			value
				.to_str()
				.expect("Can't convert CUPS option value to String")
		)
	}

	#[test]
	fn if_many_finishing_then_comma_separated_cups_finishing_constants() {
		// Several finishing are present:
		let finishings = vec![Finishing::Staple, Finishing::Bind, Finishing::Punch];

		// The CUPS option value should be comma separated string of respective integer constants:
		let value = finishings.get_cups_option_value();
		let expected_str = format!(
			"{},{},{}",
			finishings[0]
				.get_cups_option_value()
				.to_str()
				.expect("Can't convert CUPS const to String"),
			finishings[1]
				.get_cups_option_value()
				.to_str()
				.expect("Can't convert CUPS const to String"),
			finishings[2]
				.get_cups_option_value()
				.to_str()
				.expect("Can't convert CUPS const to String")
		);
		let expected_c_str =
			CString::new(expected_str.clone()).expect("Can't convert expected string to CString");

		assert_eq!(
			expected_c_str.as_c_str(),
			value.deref(),
			// message:
			"Finishings should have value '{}', was: '{}'",
			expected_str,
			value
				.to_str()
				.expect("Can't convert CUPS option value to String")
		)
	}
}
