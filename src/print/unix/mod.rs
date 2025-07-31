use std::{borrow, ffi};

use crate::print::unix::job::FatPointerMut;

pub mod cups;
pub mod dest;
pub mod job;
pub mod native;
pub mod options;

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

/// Returns a slice viewing into the specified fat pointer.
/// The pointer must be valid.
unsafe fn fat_ptr_to_slice<'a, T>(ptr: &FatPointerMut<T>) -> &'a [T] {
	if ptr.num > 0 {
		unsafe { std::slice::from_raw_parts(ptr.ptr, ptr.num as usize) }
	} else {
		&mut []
	}
}

/// Returns a mutable slice viewing into the specified fat pointer.
/// The pointer must be valid.
unsafe fn fat_ptr_to_slice_mut<'a, T>(ptr: &FatPointerMut<T>) -> &'a mut [T] {
	if ptr.num > 0 {
		unsafe { std::slice::from_raw_parts_mut(ptr.ptr, ptr.num as usize) }
	} else {
		&mut []
	}
}
