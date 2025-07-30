use std::ffi::c_int;

#[derive(Clone, Copy)]
pub struct CopiesInt(pub c_int);
#[derive(Clone, Copy)]
pub struct NumberUpInt(pub c_int);

pub struct PrintOptions {
	/// Amount of copies.
	pub copies: CopiesInt,
	/// Finishing processes such as stapling, folding, etc.
	pub finishings: Vec<Finishing>,
	/// Media size.
	pub media_size: MediaSize,
	/// The source where the media is pulled from.
	pub media_source: MediaSource,
	/// Type of media, such as photo paper, matte paper, etc.
	pub media_type: MediaType,
	/// Number of document pages per media side.
	pub number_up: NumberUpInt,
	/// Orientation of document pages on the media.
	pub orientation: Orientation,
	/// Color of the output.
	pub color_mode: ColorMode,
	/// Quality of output.
	pub quality: Quality,
	/// Determines single-sided or duplex printing.
	pub sides_mode: SidesMode,
}

pub enum Finishing {
	Bind,
	Cover,
	Fold,
	Punch,
	Staple,
	Trim,
}
pub enum MediaSize {
	A3,
	A3Plus,
	A4,
	A5,
	A6,
	Index3x5,
	Index4x6,
	Inches5x7,
	GovtLetter8x10,
	Envelope10,
	EnvelopeDL,
	Legal,
	Letter,
	Photo3R,
	Tabloid,
}

pub enum MediaSource {
	Auto,
	Manual,
}

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

pub enum Orientation {
	Portrait,
	Landscape,
}

pub enum ColorMode {
	Auto,
	Monochrome,
	Color,
}

pub enum Quality {
	Draft,
	Normal,
	High,
}

pub enum SidesMode {
	OneSided,
	TwoSidedPortrait,
	TwoSidedLandscape,
}
