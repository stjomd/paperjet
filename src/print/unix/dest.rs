use std::ffi::CStr;
use std::ops::Deref;
use std::ops::DerefMut;
use std::ptr;

use crate::print::unix::FatPointerMut;
use crate::print::unix::cups;

// NOTE: the point of these structs/wrappers is to adapt unsafe bindings to safe Rust types.
// It is IMPORTANT that these structs do not expose any public initializers, public constructors
// that accept pointers or references, even transitively, or make the internal fields modifiable,
// since this can endanger safety.
//
// These wrappers are 'chained':
// CupsDestinations::new constructs an instance with a valid pointer to an array of destinations.
// CupsDestinations::get uses this valid pointer to return a particular, valid CupsDestination,
// 		which thus stores a safe reference. It can't outlive CupsDestinations.
// CupsDestination::get_info uses this reference to return a valid CupsDestinationInfo, which
// 		itself stores a safe reference. It can't outlive CupsDestination, and thus CupsDestinations.
//
// Because CUPS works with mutable pointers a lot, we need to store mutable references and thus
// often require a mutable reference in the functions.

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
	pub fn get(&mut self, index: usize) -> Option<CupsDestination> {
		// SAFETY: `self.0` is a valid fat pointer, pointing to memory allocated by CUPS.
		// It remains valid until `cupsFreeDests` is called, which happens on drop.
		let reference_dest = unsafe {
			let ptr = self.0.get_at(index)?;
			&mut *ptr
		};
		Some(CupsDestination::new(reference_dest))
	}
}
impl Drop for CupsDestinations {
	fn drop(&mut self) {
		if self.0.is_null() {
			return;
		}
		// SAFETY: `self.dests` is a valid fat pointer, pointing to memory allocated by CUPS.
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
		let slice = unsafe { self.0.as_slice_mut() };
		slice.iter_mut().map(CupsDestination::new)
	}
}

// MARK: - Destination

pub struct CupsDestination<'a>(&'a mut cups::cups_dest_t, CupsDestinationInfo<'a>);
impl<'a> CupsDestination<'a> {
	/// Wraps a valid destination in this struct.
	fn new(dest: &'a mut cups::cups_dest_t) -> Self {
		let info = CupsDestinationInfo::new(dest);
		Self(dest, info)
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
			// this is a valid pointer and can be safely casted to a reference.
			let reference = unsafe { &mut *dest };
			Some(Self::new(reference))
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
			// this is a valid pointer and can be safely casted to a reference.
			let reference = unsafe { &mut *dest };
			Some(Self::new(reference))
		}
	}
	/// Returns this destination's information.
	pub fn get_info(&mut self) -> &mut CupsDestinationInfo<'a> {
		&mut self.1
	}
}
impl<'a> Deref for CupsDestination<'a> {
	type Target = cups::cups_dest_t;
	fn deref(&self) -> &Self::Target {
		self.0
	}
}
impl<'a> DerefMut for CupsDestination<'a> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		self.0
	}
}

// MARK: - Destination Info

/// A struct representing CUPS information for a particular destination.
pub struct CupsDestinationInfo<'a>(&'a mut cups::cups_dinfo_t);
impl<'a> CupsDestinationInfo<'a> {
	fn new(destination: &mut cups::cups_dest_t) -> Self {
		let reference = unsafe {
			let ptr = cups::cupsCopyDestInfo(cups::consts::http::CUPS_HTTP_DEFAULT, destination);
			&mut *ptr
		};
		CupsDestinationInfo(reference)
	}
}
impl<'a> Drop for CupsDestinationInfo<'a> {
	fn drop(&mut self) {
		// SAFETY: `self.0` is a valid pointer returned by CUPS and obtained in `Self::new`.
		unsafe { cups::cupsFreeDestInfo(self.0) };
	}
}
impl<'a> Deref for CupsDestinationInfo<'a> {
	type Target = cups::cups_dinfo_t;
	fn deref(&self) -> &Self::Target {
		self.0
	}
}
impl<'a> DerefMut for CupsDestinationInfo<'a> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		self.0
	}
}

#[cfg(test)]
mod tests {
	use std::ffi;

	use crate::print::unix::dest::CupsDestinations;
	use crate::print::unix::{FatPointerMut, cups};

	#[test]
	fn if_no_destinations_then_get_always_none() {
		// This CUPS dests array is empty:
		let mut dests = [];
		let fptr: FatPointerMut<cups::cups_dest_t> = FatPointerMut {
			size: dests.len() as ffi::c_int,
			ptr: &mut dests as *mut _,
		};
		let mut cups_destinations = CupsDestinations(fptr);
		// Any call to .get() should return None:
		assert!(cups_destinations.get(0).is_none());
		assert!(cups_destinations.get(1).is_none());
		assert!(cups_destinations.get(usize::MAX).is_none());
	}
}
