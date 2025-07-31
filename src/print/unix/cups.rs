#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[allow(unused)]
mod bindings {
	include!(concat!(env!("OUT_DIR"), "/cups-bindings.rs"));
}
pub use bindings::*;

// These constants are macro-ed (#define)...
pub mod consts {
	use super::*;
	use std::ffi::{CStr, c_char, c_int};
	use std::ptr::null_mut;

	macro_rules! const_cstr {
		($id:ident = $e:expr) => {
			pub const $id: &CStr = $e;
		};
	}

	pub fn bool(value: bool) -> c_int {
		if value { 1 } else { 0 }
	}

	pub mod http {
		use super::*;
		pub const CUPS_HTTP_DEFAULT: *mut http_t = null_mut();
	}

	pub mod opts {
		use super::*;
		const_cstr!(CUPS_COPIES = c"copies");
		const_cstr!(CUPS_FINISHINGS = c"finishings");
		const_cstr!(CUPS_MEDIA = c"media");
		const_cstr!(CUPS_MEDIA_SOURCE = c"media-source");
		const_cstr!(CUPS_MEDIA_TYPE = c"media-type");
		const_cstr!(CUPS_NUMBER_UP = c"number-up");
		const_cstr!(CUPS_ORIENTATION = c"orientation-requested");
		const_cstr!(CUPS_PRINT_COLOR_MODE = c"print-color-mode");
		const_cstr!(CUPS_PRINT_QUALITY = c"print-quality");
		const_cstr!(CUPS_SIDES = c"sides");

		pub mod values {
			use super::*;
			// Finishings
			const_cstr!(CUPS_FINISHINGS_BIND = c"7");
			const_cstr!(CUPS_FINISHINGS_COVER = c"6");
			const_cstr!(CUPS_FINISHINGS_FOLD = c"10");
			const_cstr!(CUPS_FINISHINGS_NONE = c"3");
			const_cstr!(CUPS_FINISHINGS_PUNCH = c"5");
			const_cstr!(CUPS_FINISHINGS_STAPLE = c"4");
			const_cstr!(CUPS_FINISHINGS_TRIM = c"11");
			// Media
			const_cstr!(CUPS_MEDIA_3X5 = c"na_index-3x5_3x5in");
			const_cstr!(CUPS_MEDIA_4X6 = c"na_index-4x6_4x6in");
			const_cstr!(CUPS_MEDIA_5X7 = c"na_5x7_5x7in");
			const_cstr!(CUPS_MEDIA_8X10 = c"na_govt-letter_8x10in");
			const_cstr!(CUPS_MEDIA_A3 = c"iso_a3_297x420mm");
			const_cstr!(CUPS_MEDIA_A4 = c"iso_a4_210x297mm");
			const_cstr!(CUPS_MEDIA_A5 = c"iso_a5_148x210mm");
			const_cstr!(CUPS_MEDIA_A6 = c"iso_a6_105x148mm");
			const_cstr!(CUPS_MEDIA_ENV10 = c"na_number-10_4.125x9.5in");
			const_cstr!(CUPS_MEDIA_ENVDL = c"iso_dl_110x220mm");
			const_cstr!(CUPS_MEDIA_LEGAL = c"na_legal_8.5x14in");
			const_cstr!(CUPS_MEDIA_LETTER = c"na_letter_8.5x11in");
			const_cstr!(CUPS_MEDIA_PHOTO_L = c"oe_photo-l_3.5x5in");
			const_cstr!(CUPS_MEDIA_SUPERBA3 = c"na_super-b_13x19in");
			const_cstr!(CUPS_MEDIA_TABLOID = c"na_ledger_11x17in");
			// Media source
			const_cstr!(CUPS_MEDIA_SOURCE_AUTO = c"auto");
			const_cstr!(CUPS_MEDIA_SOURCE_MANUAL = c"manual");
			// Media type
			const_cstr!(CUPS_MEDIA_TYPE_AUTO = c"auto");
			const_cstr!(CUPS_MEDIA_TYPE_ENVELOPE = c"envelope");
			const_cstr!(CUPS_MEDIA_TYPE_LABELS = c"labels");
			const_cstr!(CUPS_MEDIA_TYPE_LETTERHEAD = c"stationery-letterhead");
			const_cstr!(CUPS_MEDIA_TYPE_PHOTO = c"photographic");
			const_cstr!(CUPS_MEDIA_TYPE_PHOTO_GLOSSY = c"photographic-glossy");
			const_cstr!(CUPS_MEDIA_TYPE_PHOTO_MATTE = c"photographic-matte");
			const_cstr!(CUPS_MEDIA_TYPE_PLAIN = c"stationery");
			const_cstr!(CUPS_MEDIA_TYPE_TRANSPARENCY = c"transparency");
			// Orientation
			const_cstr!(CUPS_ORIENTATION_PORTRAIT = c"3");
			const_cstr!(CUPS_ORIENTATION_LANDSCAPE = c"4");
			// Color mode
			const_cstr!(CUPS_PRINT_COLOR_MODE_AUTO = c"auto");
			const_cstr!(CUPS_PRINT_COLOR_MODE_MONOCHROME = c"monochrome");
			const_cstr!(CUPS_PRINT_COLOR_MODE_COLOR = c"color");
			// Quality
			const_cstr!(CUPS_PRINT_QUALITY_DRAFT = c"3");
			const_cstr!(CUPS_PRINT_QUALITY_NORMAL = c"4");
			const_cstr!(CUPS_PRINT_QUALITY_HIGH = c"5");
			// Sides
			const_cstr!(CUPS_SIDES_ONE_SIDED = c"one-sided");
			const_cstr!(CUPS_SIDES_TWO_SIDED_PORTRAIT = c"two-sided-long-edge");
			const_cstr!(CUPS_SIDES_TWO_SIDED_LANDSCAPE = c"two-sided-short-edge");
		}
	}

	pub mod format {
		use super::*;
		// const_cstr!(CUPS_FORMAT_AUTO = c"application/octet-stream")
		pub const CUPS_FORMAT_AUTO: *const c_char = c"application/octet-stream".as_ptr();
	}
}
