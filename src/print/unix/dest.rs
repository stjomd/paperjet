use std::ffi::CStr;
use std::ops::Deref;
use std::ops::DerefMut;
use std::ptr;

use crate::print::unix::FatPointerMut;
use crate::print::unix::cups;

// NOTE: the point of these structs/wrappers is to adapt unsafe bindings to safe Rust types.
// It is IMPORTANT that these structs do not rely on caller input during initialization if
// performing unsafe operations. They also MUST NOT mutably expose their contents.
//
// CupsDestinations<'a> 'contains' any number of CupsDestination<'a>, passing down the lifetime,
// so that a CupsDestination doesn't outlive its parent CupsDestinations.
//
// These structs MUST only expose safe constructors that do not accept references or pointers.

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
		unsafe {
			let ptr = self.0.get_at(index)?;
			let reference = &mut *ptr;
			Some(CupsDestination::new(reference))
		}
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

pub struct CupsDestination<'a>(&'a mut cups::cups_dest_t);
impl<'a> CupsDestination<'a> {
	/// Wraps a valid destination in this struct.
	///
	/// # Safety
	/// `dest` must be a valid reference pointing to a [`cups::cups_dest_t`] managed by CUPS.
	unsafe fn new(dest: &'a mut cups::cups_dest_t) -> Self {
		Self(dest)
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
	/// Retrieves destination info from CUPS and wraps the pointer in this struct.
	pub fn new(destination: &mut CupsDestination) -> Option<Self> {
		// SAFETY: `destination` is wrapped in CupsDestination, and thus contains a valid reference.
		let ptr = unsafe {
			cups::cupsCopyDestInfo(
				cups::consts::http::CUPS_HTTP_DEFAULT,
				destination.deref_mut(),
			)
		};
		if ptr.is_null() {
			return None;
		}
		// SAFETY: `cupsCopyDestInfo` might return a null pointer, which was checked above.
		// Thus at this point, the pointer is valid, and can be casted to a reference.
		let reference = unsafe { &mut *ptr };
		Some(CupsDestinationInfo(reference))
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
	use crate::print::unix::dest::CupsDestinations;
	use crate::print::unix::{FatPointerMut, cups};

	#[test]
	fn if_no_destinations_then_get_always_none() {
		// This CUPS dests array is empty:
		let mut dests = [];
		let fptr: FatPointerMut<cups::cups_dest_t> = FatPointerMut {
			size: dests.len() as _,
			ptr: &mut dests as _,
		};
		let mut cups_destinations = CupsDestinations(fptr);
		// Any call to .get() should return None:
		assert!(cups_destinations.get(0).is_none());
		assert!(cups_destinations.get(1).is_none());
		assert!(cups_destinations.get(usize::MAX).is_none());
	}
}
