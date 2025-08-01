use std::ptr;

use crate::print::unix::job::FatPointerMut;
use crate::print::unix::{cups, fat_ptr_to_slice, fat_ptr_to_slice_mut};

// MARK: - Destinations Array

/// A struct representing an array of CUPS destinations.
pub struct CupsDestinations {
	/// A fat pointer to the array of destinations allocated by CUPS.
	dests: FatPointerMut<cups::cups_dest_t>,
}

impl CupsDestinations {
	/// Creates a new instance of this struct, retrieving CUPS destinations.
	pub fn new() -> Self {
		let mut dests_ptr = ptr::null_mut();
		// SAFETY: `cupsGetDests`` accepts a pointer to `*mut cups_dest_t`, allocates an array,
		// populates the passed in pointer with a valid pointer to the array, and returns the number
		// of elemenets. These are valid until `cupsFreeDests` is called.
		let dests_num = unsafe { cups::cupsGetDests(&mut dests_ptr) };
		Self {
			dests: FatPointerMut {
				num: dests_num,
				ptr: dests_ptr,
			},
		}
	}
	/// Returns an immutable view into the CUPS destinations as a slice.
	pub fn as_slice(&self) -> &[cups::cups_dest_t] {
		// SAFETY: `self.dests` is a valid fat pointer, pointing to memory allocated by CUPS.
		// It remains valid until `cupsFreeDests` is called.
		unsafe { fat_ptr_to_slice(&self.dests) }
	}
	/// Returns a mutable view into the CUPS destinations as a slice.
	fn as_slice_mut(&mut self) -> &mut [cups::cups_dest_t] {
		// SAFETY: `self.dests` is a valid fat pointer, pointing to memory allocated by CUPS.
		// It remains valid until `cupsFreeDests` is called.
		unsafe { fat_ptr_to_slice_mut(&self.dests) }
	}
	/// Returns a destination at the specified index, or [`None`] if the index is invalid.
	pub fn get_mut(&mut self, index: usize) -> Option<&mut cups::cups_dest_t> {
		self.as_slice_mut().get_mut(index)
	}
}

impl Drop for CupsDestinations {
	fn drop(&mut self) {
		// SAFETY: `self.dests` is a valid fat pointer, pointing to memory allocated by CUPS.
		// It remains valid until `cupsFreeDests` is called, which is now.
		unsafe { cups::cupsFreeDests(self.dests.num, self.dests.ptr) };
		// Seems like we don't have to drop the options on each destination ourselves (causes
		// occasional double frees), and they're dropped by CUPS along with this call.
	}
}
