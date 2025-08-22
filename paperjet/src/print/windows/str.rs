use std::fmt::Display;
use std::iter;

use windows::core::{PCWSTR, PWSTR};

/// A wide (16-bit Unicode), null-terminated string.
#[derive(Clone, Debug)]
pub struct WideString {
	bytes: Vec<u16>,
}

impl WideString {
	/// Returns a [`PCWSTR`] (pointer to constant wide string) to this string.
	pub fn as_pcwstr(&self) -> PCWSTR {
		PCWSTR::from_raw(self.bytes.as_ptr())
	}
	/// Returns a [`PWSTR`] (pointer to wide string) to this string.
	pub fn as_pwstr(&mut self) -> PWSTR {
		PWSTR::from_raw(self.bytes.as_mut_ptr())
	}
}

impl From<&str> for WideString {
	fn from(value: &str) -> Self {
		Self {
			bytes: value.encode_utf16().chain(iter::once(0)).collect(),
		}
	}
}

impl From<WideString> for String {
	fn from(value: WideString) -> Self {
		Self::from(&value)
	}
}
impl From<&WideString> for String {
	fn from(value: &WideString) -> Self {
		let unterminated_bytes = &value.bytes[0..&value.bytes.len() - 1];
		Self::from_utf16_lossy(unterminated_bytes)
	}
}

impl Display for WideString {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", String::from(self))
	}
}
