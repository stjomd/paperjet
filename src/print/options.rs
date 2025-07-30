use std::ffi::c_int;

#[derive(Debug, Clone, Copy)]
pub struct CopiesInt(pub c_int);
#[derive(Debug, Clone, Copy)]
pub struct NumberUpInt(pub c_int);

/// A struct that defines options for a print job.
#[derive(Debug, Default)]
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

#[derive(Debug)]
pub enum Finishing {
	Bind,
	Cover,
	Fold,
	Punch,
	Staple,
	Trim,
}

#[derive(Debug)]
pub enum MediaSize {
	A3,
	A3Plus,
	A4,
	A5,
	A6,
	Index3x5,
	Index4x6,
	Index5x7,
	GovtLetter8x10,
	Envelope10,
	EnvelopeDL,
	Legal,
	Letter,
	Photo3R,
	Tabloid,
}

#[derive(Debug)]
pub enum MediaSource {
	Auto,
	Manual,
}

#[derive(Debug)]
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

#[derive(Debug)]
pub enum Orientation {
	Portrait,
	Landscape,
}

#[derive(Debug)]
pub enum ColorMode {
	Auto,
	Monochrome,
	Color,
}

#[derive(Debug)]
pub enum Quality {
	Draft,
	Normal,
	High,
}

#[derive(Debug)]
pub enum SidesMode {
	OneSided,
	TwoSidedPortrait,
	TwoSidedLandscape,
}
