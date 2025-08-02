use std::fs::{self, File};
use std::path::PathBuf;

use printrs::Printer;

/// The subdirectory inside the system-specific cache directory.
const SNAPSHOT_SUBDIR_NAME: &str = concat!("com.github.stjomd.", env!("CARGO_BIN_NAME"));
/// The name of the snapshot file.
const SNAPSHOT_FILE_NAME: &str = "printers.snapshot";

/// Saves a snapshot of the specified printers.
pub fn save(printers: &[Printer]) {
	let Some(cache_dir) = get_snapshot_dir(SNAPSHOT_SUBDIR_NAME) else {
		return;
	};
	let Some(mut file) = File::create(cache_dir.join(SNAPSHOT_FILE_NAME)).ok() else {
		return;
	};

	let snapshot = printers
		.iter()
		.map(PrinterSnapshot::from)
		.collect::<Vec<_>>();

	let _ = bincode::encode_into_std_write(snapshot, &mut file, bincode::config::standard());
}

pub fn open() -> Option<Vec<PrinterSnapshot>> {
	let dir = get_snapshot_dir(SNAPSHOT_SUBDIR_NAME)?;
	let mut file = File::open(dir.join(SNAPSHOT_FILE_NAME)).ok()?;
	bincode::decode_from_std_read(&mut file, bincode::config::standard()).ok()
}

// MARK: - Snapshot types

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

// MARK: - File System

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
