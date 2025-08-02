use std::ffi::c_int;

/// A struct that defines options for a print job.
#[derive(Clone, Debug, Default)]
pub struct PrintOptions {
	/// Amount of copies.
	pub copies: Option<CopiesInt>,
	/// Finishing processes such as stapling, folding, etc.
	pub finishings: Option<Vec<Finishing>>,
	/// Media size.
	pub media_size: Option<MediaSize>,
	/// The source where the media is pulled from.
	pub media_source: Option<MediaSource>,
	/// Type of media, such as photo paper, matte paper, etc.
	pub media_type: Option<MediaType>,
	/// Number of document pages per media side.
	pub number_up: Option<NumberUpInt>,
	/// Orientation of document pages on the media.
	pub orientation: Option<Orientation>,
	/// Color of the output.
	pub color_mode: Option<ColorMode>,
	/// Quality of output.
	pub quality: Option<Quality>,
	/// Determines single-sided or duplex printing.
	pub sides_mode: Option<SidesMode>,
}

pub trait PrintOption {
	fn get_name() -> &'static str;
	fn to_human_string(&self) -> String;
}

macro_rules! impl_print_option {
	($type:ty => $name:expr) => {
		impl PrintOption for $type {
			fn get_name() -> &'static str {
				$name
			}
			fn to_human_string(&self) -> String {
				format!("{self}")
			}
		}
	};
}

// MARK: - Conrete Options

#[derive(Clone, Copy, Debug)]
pub struct CopiesInt(pub c_int);
impl From<c_int> for CopiesInt {
	fn from(value: c_int) -> Self {
		Self(value)
	}
}
impl PrintOption for CopiesInt {
	fn get_name() -> &'static str {
		"Copies"
	}
	fn to_human_string(&self) -> String {
		self.0.to_string()
	}
}

#[derive(Clone, Copy, Debug)]
pub struct NumberUpInt(pub c_int);
impl From<c_int> for NumberUpInt {
	fn from(value: c_int) -> Self {
		Self(value)
	}
}
impl PrintOption for NumberUpInt {
	fn get_name() -> &'static str {
		"Number Up"
	}
	fn to_human_string(&self) -> String {
		self.0.to_string()
	}
}

#[derive(Clone, Debug, clap::ValueEnum, strum::Display)]
#[strum(serialize_all = "kebab-case")]
pub enum Finishing {
	Bind,
	Cover,
	Fold,
	Punch,
	Staple,
	Trim,
}
impl_print_option!(Finishing => "Finishing");

impl PrintOption for Vec<Finishing> {
	fn get_name() -> &'static str {
		"Finishings"
	}
	fn to_human_string(&self) -> String {
		self.iter()
			.map(|f| f.to_human_string())
			.collect::<Vec<_>>()
			.join(", ")
	}
}

#[derive(Clone, Debug, clap::ValueEnum, strum::Display)]
#[strum(serialize_all = "kebab-case")]
pub enum MediaSize {
	// ISO & A3+
	A3,
	A3Plus,
	A4,
	A5,
	A6,
	// US
	GovtLetter,
	Letter,
	Legal,
	Tabloid,
	// Miscellaneous
	Index3x5,
	Index4x6,
	Index5x7,
	Envelope10,
	EnvelopeDL,
	Photo3R,
}
impl_print_option!(MediaSize => "Media Size");

#[derive(Clone, Debug, clap::ValueEnum, strum::Display)]
#[strum(serialize_all = "kebab-case")]
pub enum MediaSource {
	Auto,
	Manual,
}
impl_print_option!(MediaSource => "Media Source");

#[derive(Clone, Debug, clap::ValueEnum, strum::Display)]
#[strum(serialize_all = "kebab-case")]
pub enum MediaType {
	Auto,
	Envelope,
	Labels,
	Letterhead,
	Photo,
	PhotoGlossy,
	PhotoMatte,
	Plain,
	Transparent,
}
impl_print_option!(MediaType => "Media Type");

#[derive(Clone, Debug, clap::ValueEnum, strum::Display)]
#[strum(serialize_all = "kebab-case")]
pub enum Orientation {
	Portrait,
	Landscape,
}
impl_print_option!(Orientation => "Orientation");

#[derive(Clone, Debug, clap::ValueEnum, strum::Display)]
#[strum(serialize_all = "kebab-case")]
pub enum ColorMode {
	Auto,
	Monochrome,
	Color,
}
impl_print_option!(ColorMode => "Color Mode");

#[derive(Clone, Debug, clap::ValueEnum, strum::Display)]
#[strum(serialize_all = "kebab-case")]
pub enum Quality {
	Draft,
	Normal,
	High,
}
impl_print_option!(Quality => "Quality");

#[derive(Clone, Debug, clap::ValueEnum, strum::Display)]
#[strum(serialize_all = "kebab-case")]
pub enum SidesMode {
	OneSided,
	TwoSidedPortrait,
	TwoSidedLandscape,
}
impl_print_option!(SidesMode => "SidesMode");
