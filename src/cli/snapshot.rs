use std::fs::{self, File};
use std::path::PathBuf;
use std::time::Duration;

use anyhow::Result;

pub mod printers {
	use printrs::Printer;

	use super::*;

	/// The name of the snapshot file.
	const SNAPSHOT_FILE_NAME: &str = "printers.snapshot";
	/// The maximum duration of a valid snapshot file.
	const SNAPSHOT_MAX_AGE: Duration = Duration::new(120, 0);

	/// Saves a snapshot of the specified, sorted, printers.
	pub fn save(printers: &[Printer]) {
		super::save_all::<_, PrinterSnapshot>(printers, SNAPSHOT_FILE_NAME);
	}
	/// Opens the snapshot file and returns its deserialized contents.
	pub fn open() -> Option<Vec<PrinterSnapshot>> {
		super::open(SNAPSHOT_FILE_NAME, SNAPSHOT_MAX_AGE)
	}

	/// Struct representing a snapshot of a printer.
	#[derive(Debug, bincode::Encode, bincode::Decode)]
	pub struct PrinterSnapshot {
		pub human_name: String,
		pub identifier: String,
	}
	impl From<&Printer> for PrinterSnapshot {
		fn from(value: &Printer) -> Self {
			Self {
				human_name: value.get_human_name().clone(),
				identifier: value.identifier.clone(),
			}
		}
	}
}

// MARK: - Snapshot Implementation

/// The subdirectory inside the system-specific cache directory.
const SNAPSHOT_SUBDIR_NAME: &str = concat!("com.github.stjomd.", env!("CARGO_BIN_NAME"));

/// Saves a snapshot of the specified items.
fn save_all<'a, T, S>(items: &'a [T], file_name: &str)
where
	S: From<&'a T> + bincode::Encode,
{
	let Some(cache_dir) = get_snapshot_dir(SNAPSHOT_SUBDIR_NAME) else {
		return;
	};
	let Some(mut file) = File::create(cache_dir.join(file_name)).ok() else {
		return;
	};

	let snapshot = items.iter().map(|item| S::from(item)).collect::<Vec<_>>();
	let _ = bincode::encode_into_std_write(snapshot, &mut file, bincode::config::standard());
}

/// Opens the snapshot file and deserializes the contents.
fn open<S>(file_name: &str, max_age: Duration) -> Option<S>
where
	S: bincode::Decode<()>,
{
	let dir = get_snapshot_dir(SNAPSHOT_SUBDIR_NAME)?;
	let mut file = File::open(dir.join(file_name)).ok()?;

	let is_valid = check_is_valid(&file, max_age).unwrap_or(false);
	if !is_valid {
		return None;
	}

	bincode::decode_from_std_read(&mut file, bincode::config::standard()).ok()
}

/// Checks if the file, since last modification, has existed for less than the `max_age` specified.
fn check_is_valid(file: &File, max_age: Duration) -> Result<bool> {
	let elapsed = file.metadata()?.modified()?.elapsed()?;
	let diff = elapsed.saturating_sub(max_age);
	// if diff is zero, this means the file exists for less than `max_age`, and is thus valid
	Ok(diff == Duration::ZERO)
}

fn get_snapshot_dir(subdir_name: &str) -> Option<PathBuf> {
	let mut cache_dir = dirs::cache_dir()?;
	cache_dir.push(subdir_name);

	if let Ok(true) = cache_dir.try_exists() {
		return Some(cache_dir);
	}
	if fs::create_dir(cache_dir.clone()).is_ok() {
		return Some(cache_dir);
	}

	None
}
