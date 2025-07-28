use std::env;
use std::path::PathBuf;

fn main() {
	let target_family = std::env::var("CARGO_CFG_TARGET_FAMILY").unwrap();

	if target_family == "unix" {
		// Link CUPS
		println!("cargo:rustc-link-lib=cups");
		// Generate bindings
		cups_bindings();
	}
}

fn cups_bindings() {
	let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
	let builder = bindgen::builder().header("headers/cups.h");

	// Allowlist:
	let builder = builder
		.allowlist_function("cupsGetDests")
		.allowlist_function("cupsFreeDests");

	// Generate & write:
	let bindings = builder
		.generate()
		.expect("Unable to generate bindings for CUPS");
	bindings
		.write_to_file(out_dir.join("cups-bindings.rs"))
		.expect("Unable to write bindings for CUPS");
}
