use std::borrow::Cow;
use std::ffi::{CStr, CString};

use crate::options::{
	ColorMode, CopiesInt, Finishing, MediaSize, MediaSource, MediaType, NumberUpInt, Orientation,
	Quality, SidesMode,
};
use crate::print::unix::cups::consts::opts;

// MARK: - Option values

pub trait ToCupsOptionValue {
	fn to_cups_option_value(&self) -> Cow<'static, CStr>;
}

impl ToCupsOptionValue for CopiesInt {
	fn to_cups_option_value(&self) -> Cow<'static, CStr> {
		let string = self.0.to_string();
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
