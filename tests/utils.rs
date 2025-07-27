#![allow(unused)]

macro_rules! assert_matches {
	($e:expr, $p:pat) => {
		assert!(matches!($e, $p))
	};
	($e:expr, $p:pat, $m:expr) => {
		assert!(matches!($e, $p), $m)
	};
}
pub(crate) use assert_matches;

#[cfg(target_family = "unix")]
mod dummy {
	use std::process::Command;

	/// Represents a dummy printer on the system.
	/// Creating an instance with [`Self::try_new`] registers a destination
	/// with the specified name with CUPS.
	/// When dropped, the destination will be removed.
	pub struct DummyPrinter {
		pub name: String,
		pub device_uri: String,
	}

	impl DummyPrinter {
		/// Creates a new printer in the system with a random name and a URI to `/dev/null`.
		pub fn try_new() -> Result<Self, std::io::Error> {
			let name = "printrs-test-".to_owned() + &uuid::Uuid::new_v4().to_string();
			let device_uri = "file:/dev/null".to_owned();
			let output = Command::new("lpadmin")
				.args(["-p", &name])
				.args(["-v", &device_uri])
				.output()?;
			println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
			println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
			Ok(Self { name, device_uri })
		}
	}

	impl Drop for DummyPrinter {
		/// Removes the printer from the system.
		fn drop(&mut self) {
			let result = Command::new("lpadmin").args(["-x", &self.name]).output();
			if std::thread::panicking() {
				return;
			}

			let error_msg = format!("Could not drop {} with lpadmin", self.name);
			let output = result.expect(&error_msg);

			println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
			println!("stderr: {}", String::from_utf8_lossy(&output.stderr));

			if !output.status.success() {
				panic!(
					"Could not drop {} with lpadmin:\n- exit code: {}\n- stderr: {}",
					self.name,
					output.status.code().unwrap_or(-1),
					String::from_utf8_lossy(&output.stderr)
				)
			}
		}
	}
}
#[cfg(target_family = "unix")]
pub use dummy::*;
