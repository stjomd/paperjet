use std::borrow::Cow;
use std::ffi::{CStr, CString};

use crate::options::{
	ColorMode, CopiesInt, Finishing, MediaSize, MediaSource, MediaType, NumberUpInt, Orientation,
	Quality, SidesMode,
};
use crate::print::unix::cups::consts::opts;

// MARK: - Option values

/// A trait that designates an option that can be converted to a CUPS option value string.
pub trait ToCupsOptionValue {
	/// Converts the option's value to a string accepted by CUPS.
	/// Returns either a borrowed or an owned value inside a [`Cow`] pointer.
	fn to_cups_option_value(&self) -> Cow<'static, CStr>;
}

impl ToCupsOptionValue for CopiesInt {
	fn to_cups_option_value(&self) -> Cow<'static, CStr> {
		let string = self.0.to_string();
		// SAFETY: `string` is built from `self.0`, which is a C integer, and thus contains only bytes
		// which correspond to digit characters, and no 0 bytes.
		let c_string = CString::new(string).expect("Could not convert copies to CString");
		Cow::Owned(c_string)
	}
}

impl ToCupsOptionValue for Finishing {
	fn to_cups_option_value(&self) -> Cow<'static, CStr> {
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
impl ToCupsOptionValue for Vec<Finishing> {
	fn to_cups_option_value(&self) -> Cow<'static, CStr> {
		if self.is_empty() {
			return Cow::Borrowed(opts::values::CUPS_FINISHINGS_NONE);
		}
		// We want a comma-separated string here:
		let bytes = self
			.iter()
			.map(|finishing| finishing.to_cups_option_value())
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

impl ToCupsOptionValue for MediaSize {
	fn to_cups_option_value(&self) -> Cow<'static, CStr> {
		Cow::Borrowed(match self {
			MediaSize::A3 => opts::values::CUPS_MEDIA_A3,
			MediaSize::A3Plus => opts::values::CUPS_MEDIA_SUPERBA3,
			MediaSize::A4 => opts::values::CUPS_MEDIA_A4,
			MediaSize::A5 => opts::values::CUPS_MEDIA_A5,
			MediaSize::A6 => opts::values::CUPS_MEDIA_A6,
			MediaSize::Index3x5 => opts::values::CUPS_MEDIA_3X5,
			MediaSize::Index4x6 => opts::values::CUPS_MEDIA_4X6,
			MediaSize::Inches5x7 => opts::values::CUPS_MEDIA_5X7,
			MediaSize::GovtLetter8x10 => opts::values::CUPS_MEDIA_8X10,
			MediaSize::Envelope10 => opts::values::CUPS_MEDIA_ENV10,
			MediaSize::EnvelopeDL => opts::values::CUPS_MEDIA_ENVDL,
			MediaSize::Legal => opts::values::CUPS_MEDIA_LEGAL,
			MediaSize::Letter => opts::values::CUPS_MEDIA_LETTER,
			MediaSize::Photo3R => opts::values::CUPS_MEDIA_PHOTO_L,
			MediaSize::Tabloid => opts::values::CUPS_MEDIA_TABLOID,
		})
	}
}

impl ToCupsOptionValue for MediaSource {
	fn to_cups_option_value(&self) -> Cow<'static, CStr> {
		Cow::Borrowed(match self {
			MediaSource::Auto => opts::values::CUPS_MEDIA_SOURCE_AUTO,
			MediaSource::Manual => opts::values::CUPS_MEDIA_SOURCE_MANUAL,
		})
	}
}

impl ToCupsOptionValue for MediaType {
	fn to_cups_option_value(&self) -> Cow<'static, CStr> {
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

impl ToCupsOptionValue for NumberUpInt {
	fn to_cups_option_value(&self) -> Cow<'static, CStr> {
		let string = self.0.to_string();
		// SAFETY: `string` is built from `self.0`, which is a C integer, and thus contains only bytes
		// which correspond to digit characters, and no 0 bytes.
		let c_string = CString::new(string).expect("Could not convert number up to CString");
		Cow::Owned(c_string)
	}
}

impl ToCupsOptionValue for Orientation {
	fn to_cups_option_value(&self) -> Cow<'static, CStr> {
		Cow::Borrowed(match self {
			Orientation::Portrait => opts::values::CUPS_ORIENTATION_PORTRAIT,
			Orientation::Landscape => opts::values::CUPS_ORIENTATION_LANDSCAPE,
		})
	}
}

impl ToCupsOptionValue for ColorMode {
	fn to_cups_option_value(&self) -> Cow<'static, CStr> {
		Cow::Borrowed(match self {
			ColorMode::Auto => opts::values::CUPS_PRINT_COLOR_MODE_AUTO,
			ColorMode::Monochrome => opts::values::CUPS_PRINT_COLOR_MODE_MONOCHROME,
			ColorMode::Color => opts::values::CUPS_PRINT_COLOR_MODE_COLOR,
		})
	}
}

impl ToCupsOptionValue for Quality {
	fn to_cups_option_value(&self) -> Cow<'static, CStr> {
		Cow::Borrowed(match self {
			Quality::Draft => opts::values::CUPS_PRINT_QUALITY_DRAFT,
			Quality::Normal => opts::values::CUPS_PRINT_QUALITY_NORMAL,
			Quality::High => opts::values::CUPS_PRINT_QUALITY_HIGH,
		})
	}
}

impl ToCupsOptionValue for SidesMode {
	fn to_cups_option_value(&self) -> Cow<'static, CStr> {
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
	use crate::print::unix::options::ToCupsOptionValue;

	#[test]
	fn if_empty_finishings_then_cups_finishings_none() {
		// Finishings are empty:
		let finishings: Vec<Finishing> = Vec::new();

		// The CUPS option value should be CUPS_FINISHINGS_NONE:
		let value = finishings.to_cups_option_value();

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
		let value = finishings.to_cups_option_value();

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
		let value = finishings.to_cups_option_value();
		let expected_str = format!(
			"{},{},{}",
			finishings[0]
				.to_cups_option_value()
				.to_str()
				.expect("Can't convert CUPS const to String"),
			finishings[1]
				.to_cups_option_value()
				.to_str()
				.expect("Can't convert CUPS const to String"),
			finishings[2]
				.to_cups_option_value()
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
