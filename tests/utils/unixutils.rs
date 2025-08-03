use std::{env, process};

/// Represents a fake printer on the system.
/// Creating an instance with [`Self::try_new`] registers a new dest with CUPS.
/// When dropped, the destination will be removed.
pub struct FakePrinter {
	pub name: String,
	pub device_uri: String,
}

impl FakePrinter {
	/// Creates a new printer in the system with a random name and a URI to `/dev/null`.
	pub fn try_new() -> Result<Self, std::io::Error> {
		let name = "printrs-test-".to_owned() + &uuid::Uuid::new_v4().to_string();
		let device_uri = "file:/dev/null".to_owned();
		lpadmin()
			.args(["-p", &name])
			.args(["-v", &device_uri])
			.arg("-E")
			.output()?;
		eprintln!("Created fake printer {name}");
		Ok(Self { name, device_uri })
	}
}

impl Drop for FakePrinter {
	/// Removes the printer from the system.
	fn drop(&mut self) {
		let result = lpadmin().args(["-x", &self.name]).output();
		let Ok(output) = result else {
			eprintln!("Could not drop {} with lpadmin", self.name);
			return;
		};

		if !output.status.success() {
			eprintln!(
				"Could not drop {} with lpadmin:\n- exit code: {}\n- stderr: {}",
				self.name,
				output.status.code().unwrap_or(-1),
				String::from_utf8_lossy(&output.stderr)
			);
		} else {
			eprintln!("Dropped fake printer {}", self.name);
		}
	}
}

/// Constructs an instance of either `lpadmin` or `sudo lpadmin` command.
fn lpadmin() -> process::Command {
	match env::var("USE_SUDO_LPADMIN") {
		Ok(_) => {
			let mut _command = process::Command::new("sudo");
			_command.arg("lpadmin");
			_command
		}
		Err(_) => process::Command::new("lpadmin"),
	}
}
