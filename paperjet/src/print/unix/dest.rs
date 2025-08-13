use std::ffi::CStr;
use std::marker::PhantomData;
use std::ops::Deref;
use std::ops::DerefMut;
use std::ptr;

use crate::print::unix::cups;
use crate::print::util::FatPointerMut;

// NOTE: this file contains safe wrappers for unsafe CUPS bindings.
//
// These structs should not mutably expose their direct contents (i.e. it should not be possible
// to overwrite a pointer stored in one of these structs).
//
// Where appropriate, lifetimes are tied (CupsDestination cannot outlive CupsDestinations for
// example, if we use the get method on CupsDestinations).

// MARK: - Destinations Array

/// A struct representing an array of CUPS destinations.
pub struct CupsDestinations(FatPointerMut<cups::cups_dest_t>);
impl CupsDestinations {
	/// Creates a new instance of this struct, retrieving CUPS destinations.
	#[allow(clippy::new_without_default)]
	pub fn new() -> Self {
		let mut dests_ptr = ptr::null_mut();
		// SAFETY: `cupsGetDests` accepts a pointer to `*mut cups_dest_t`, allocates an array,
		// populates the passed in pointer with a valid pointer to the array, and returns the number
		// of elemenets. These are valid until `cupsFreeDests` is called on drop.
		let dests_num =
			unsafe { cups::cupsGetDests2(cups::consts::http::CUPS_HTTP_DEFAULT, &mut dests_ptr) };
		Self(FatPointerMut {
			size: dests_num,
			ptr: dests_ptr,
		})
	}
}
impl Drop for CupsDestinations {
	fn drop(&mut self) {
		if self.0.is_null() {
			return;
		}
		// SAFETY: `self.0` is a valid fat pointer, pointing to memory allocated by CUPS.
		// It remains valid until `cupsFreeDests` is called, which is now.
		unsafe { cups::cupsFreeDests(self.0.size, self.0.ptr) };
		// Seems like we don't have to drop the options on each destination ourselves (causes
		// occasional double frees), and they're dropped by CUPS along with this call.
	}
}
impl<'a> IntoIterator for &'a mut CupsDestinations {
	type Item = CupsDestination<'a>;
	type IntoIter = std::iter::Map<
		std::slice::IterMut<'a, cups::cups_dest_t>,
		fn(&'a mut cups::cups_dest_t) -> CupsDestination<'a>,
	>;
	fn into_iter(self) -> Self::IntoIter {
		// SAFETY: `self.0` is a valid fat pointer pointing to an array of destinations, allocated
		// by CUPS, and remains valid until `cupsFreeDests` is called, which happens on drop.
		unsafe {
			let slice = self.0.as_slice_mut();
			slice.iter_mut().map(|refr| CupsDestination::new(refr))
		}
	}
}

// MARK: - Destination

pub struct CupsDestination<'a> {
	ptr: *mut cups::cups_dest_t,
	marker: PhantomData<&'a CupsDestinations>,
}
impl<'a> CupsDestination<'a> {
	/// Wraps a valid destination in this struct.
	///
	/// # Safety
	/// `dest` must be a valid reference pointing to a [`cups::cups_dest_t`] managed by CUPS.
	unsafe fn new(dest: &'a mut cups::cups_dest_t) -> Self {
		Self {
			ptr: dest,
			marker: PhantomData,
		}
	}
	/// Retrieves a destination by its name.
	pub fn new_by_name(name: &CStr) -> Option<Self> {
		// SAFETY: `cupsGetNamedDest` accepts null pointers for any of the parameters, and returns
		// a valid pointer to a destination if it is found, or a null pointer otherwise.
		let dest = unsafe {
			cups::cupsGetNamedDest(
				cups::consts::http::CUPS_HTTP_DEFAULT,
				name.as_ptr(),
				ptr::null(),
			)
		};
		if dest.is_null() {
			None
		} else {
			// SAFETY: since `cupsGetNamedDest` returns either a valid pointer to a `cups_dest_t` or
			// a null pointer, and the null pointer has been checked in the previous branch,
			// therefore this is a valid pointer and can be safely casted to a reference.
			unsafe {
				let reference = &mut *dest;
				Some(Self::new(reference))
			}
		}
	}
	/// Retrieves the default destination.
	pub fn new_default() -> Option<Self> {
		// SAFETY: `cupsGetNamedDest` accepts null pointers for any of the parameters, and returns
		// a valid pointer to a destination if it is found, or a null pointer otherwise.
		// In this case, since `name` is `ptr::null()`, the default destination will be returned.
		let dest = unsafe {
			cups::cupsGetNamedDest(
				cups::consts::http::CUPS_HTTP_DEFAULT,
				ptr::null(),
				ptr::null(),
			)
		};
		if dest.is_null() {
			None
		} else {
			// SAFETY: since `cupsGetNamedDest` returns either a valid pointer to a `cups_dest_t` or
			// a null pointer, and the null pointer has been checked in the previous branch,
			// therefore this is a valid pointer and can be safely casted to a reference.
			unsafe {
				let reference = &mut *dest;
				Some(Self::new(reference))
			}
		}
	}
	// Returns the raw mutable pointer to this destination.
	pub fn as_mut_ptr(&mut self) -> *mut cups::cups_dest_t {
		self.ptr
	}
}
impl<'a> Deref for CupsDestination<'a> {
	type Target = cups::cups_dest_t;
	fn deref(&self) -> &Self::Target {
		// SAFETY: the only safe ways to construct `CupsDestination<'a>` obtain a valid pointer from
		// CUPS, thus dereferencing is safe.
		unsafe { &*self.ptr }
	}
}
impl<'a> DerefMut for CupsDestination<'a> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		// SAFETY: the only safe ways to construct `CupsDestination<'a>` obtain a valid pointer from
		// CUPS, thus dereferencing is safe.
		unsafe { &mut *self.ptr }
	}
}

// MARK: - Destination Info

// Destination info lifetime seems to be detached from the destination itself, also implied
// by the function name `cupsCopyDestInfo` and the fact we have to free it manually.

/// A struct representing CUPS information for a particular destination.
pub struct CupsDestinationInfo(*mut cups::cups_dinfo_t);
impl CupsDestinationInfo {
	/// Retrieves destination info from CUPS and wraps the pointer in this struct.
	pub fn new(destination: &mut CupsDestination) -> Option<Self> {
		// SAFETY: `destination` is wrapped in CupsDestination, and thus contains a valid pointer.
		let ptr = unsafe {
			cups::cupsCopyDestInfo(
				cups::consts::http::CUPS_HTTP_DEFAULT,
				destination.as_mut_ptr(),
			)
		};
		if ptr.is_null() {
			return None;
		}
		Some(CupsDestinationInfo(ptr))
	}
	/// Returns the raw mutable pointer to the destination info instance.
	pub fn as_mut_ptr(&mut self) -> *mut cups::cups_dinfo_t {
		self.0
	}
}
impl Drop for CupsDestinationInfo {
	fn drop(&mut self) {
		// SAFETY: `self.ptr` is a valid pointer returned by CUPS and obtained in `Self::new`.
		unsafe { cups::cupsFreeDestInfo(self.0) };
	}
}
