mod api;
pub use api::*;

#[cfg(target_family = "unix")]
mod unix;

/// A unit struct representing the current platform.
/// There should be a platform-specific implementation of [`PlatformApi`] for this struct,
/// and a module containing this implementation should be imported above.
pub struct TargetPlatform;
/// A trait that defines the public API of this crate.
pub trait CrossPlatformApi {
	/// See [`api::get_printers()`].
	fn get_printers() -> Vec<Printer>;
}
