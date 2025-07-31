use std::{ptr, slice};

use crate::print::unix::cups;
use crate::print::unix::job::FatPointerMut;

/// A struct representing an array of CUPS destinations.
pub struct CupsDestinations {
	/// A fat pointer to the array of destinations allocated by CUPS.
	dests: FatPointerMut<cups::cups_dest_t>,
}

impl CupsDestinations {
	/// Creates a new instance of this struct, retrieving CUPS destinations.
	pub fn new() -> Self {
		let mut dests_ptr = ptr::null_mut();
		// SAFETY: cupsGetDests accepts any pointer to a pointer, and will overwrite `dests_ptr` with
		// the pointer to the allocated array.
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
		if self.dests.num > 0 {
			// SAFETY: `self.dests` contains a valid pointer & size created in the `Self::new` function.
			unsafe { slice::from_raw_parts(self.dests.ptr, self.dests.num as usize) }
		} else {
			&mut []
		}
	}
	/// Returns a mutable view into the CUPS destinations as a slice.
	fn as_slice_mut(&mut self) -> &mut [cups::cups_dest_t] {
		if self.dests.num > 0 {
			// SAFETY: `self.dests` contains a valid pointer & size created in the `Self::new` function.
			unsafe { slice::from_raw_parts_mut(self.dests.ptr, self.dests.num as usize) }
		} else {
			&mut []
		}
	}
	/// Returns a destination at the specified index, or [`None`] if the index is invalid.
	pub fn get_mut(&mut self, index: usize) -> Option<&mut cups::cups_dest_t> {
		self.as_slice_mut().get_mut(index)
	}
}

impl Drop for CupsDestinations {
	fn drop(&mut self) {
		// SAFETY: `self.dests` contains a valid pointer & size created in the `Self::new` function.
		unsafe { cups::cupsFreeDests(self.dests.num, self.dests.ptr) };
	}
}
