use std::{borrow, ffi};

/// A mutable pointer along with a size (useful for dynamic arrays).
#[derive(Clone, Copy, Debug)]
pub struct FatPointerMut<T> {
	pub size: ffi::c_int,
	pub ptr: *mut T,
}
impl<T> FatPointerMut<T> {
	/// Returns the view into the memory behind this fat pointer as a mutable slice.
	/// The pointer and the size must be valid.
	///
	/// # Safety
	/// The fat pointer must be a valid pointer; point to memory of type [`T`]; and the size
	/// must be valid.
	/// It must also fulfil all safety preconditions of [`std::slice::from_raw_parts_mut`].
	pub unsafe fn as_slice_mut(&mut self) -> &mut [T] {
		// SAFETY: precondition requires the pointer and the size are valid.
		unsafe { std::slice::from_raw_parts_mut(self.ptr, self.size as usize) }
	}
	/// Returns the element at position `index`.
	/// The pointer and the size must be valid.
	///
	/// # Safety
	/// The fat pointer must be a valid pointer; point to memory of type [`T`]; and the size
	/// must be valid.
	pub unsafe fn get_at(&self, index: usize) -> Option<*mut T> {
		if self.is_null() || index >= self.size as usize {
			None
		} else {
			// SAFETY: validity and index have been checked in the previous branch.
			let ptr = unsafe { self.ptr.add(index) };
			Some(ptr)
		}
	}
	/// Returns `true` if this fat pointer has size 0, or points to null, and `false` otherwise.
	pub fn is_null(&self) -> bool {
		self.size == 0 || self.ptr.is_null()
	}
}

/// Performs lossy conversion from a [`ffi::CStr`] into [`String`].
/// The result is either a borrowed value or an owned value.
///
/// # Safety
/// `ptr` must be a valid pointer to a valid C string.
pub unsafe fn cstr_to_str(ptr: *const ffi::c_char) -> borrow::Cow<'static, str> {
	unsafe { ffi::CStr::from_ptr(ptr).to_string_lossy() }
}
/// Constructs an owned UTF-8 string from a valid pointer to a valid C-string.
/// Invalid characters are replaced with the replacement character.
///
/// # Safety
/// `ptr` must be a valid pointer to a valid C string.
pub unsafe fn cstr_to_string(ptr: *const ffi::c_char) -> String {
	unsafe { cstr_to_str(ptr).into_owned() }
}
