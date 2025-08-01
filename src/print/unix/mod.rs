use std::{borrow, ffi};

pub mod cups;
pub mod dest;
pub mod job;
pub mod native;
pub mod options;

/// A mutable pointer along with a size (useful for dynamic arrays).
#[derive(Clone, Copy)]
pub struct FatPointerMut<T> {
	pub size: ffi::c_int,
	pub ptr: *mut T,
}
impl<T> FatPointerMut<T> {
	/// Returns the view into the memory behind this fat pointer as an immutable slice.
	/// The pointer and the size must be valid.
	pub unsafe fn as_slice(&self) -> &[T] {
		// SAFETY: precondition requires the pointer and the size are valid.
		unsafe { std::slice::from_raw_parts(self.ptr, self.size as usize) }
	}
	/// Returns the view into the memory behind this fat pointer as a mutable slice.
	/// The pointer and the size must be valid.
	pub unsafe fn as_slice_mut(&mut self) -> &mut [T] {
		// SAFETY: precondition requires the pointer and the size are valid.
		unsafe { std::slice::from_raw_parts_mut(self.ptr, self.size as usize) }
	}
}

/// Performs lossy conversion from a [`ffi::CStr`] into [`String`].
/// The result is either a borrowed value or an owned value.
unsafe fn cstr_to_str(ptr: *const ffi::c_char) -> borrow::Cow<'static, str> {
	unsafe { ffi::CStr::from_ptr(ptr).to_string_lossy() }
}
/// Constructs an owned UTF-8 string from a valid pointer to a valid C-string.
/// Invalid characters are replaced with the replacement character.
unsafe fn cstr_to_string(ptr: *const ffi::c_char) -> String {
	unsafe { cstr_to_str(ptr).into_owned() }
}
