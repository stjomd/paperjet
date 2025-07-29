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
	use std::{ffi, ptr};

	pub mod bool {
		use super::*;
		pub const TRUE: ffi::c_int = 1;
		pub const FALSE: ffi::c_int = 0;
	}

	pub mod http {
		use super::*;
		pub const CUPS_HTTP_DEFAULT: *mut http_t = ptr::null_mut();
	}

	pub mod opts {
		use super::*;
		pub const CUPS_COPIES: *const ffi::c_char = c"copies".as_ptr();
	}

	pub mod format {
		use super::*;
		pub const CUPS_FORMAT_AUTO: *const ffi::c_char = c"application/octet-stream".as_ptr();
	}
}
