use std::env;
use std::path::PathBuf;

fn main() {
	// Link CUPS
	println!("cargo:rustc-link-lib=cups");
	// Generate bindings
	cups_bindings();
}

fn cups_bindings() {
	let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

	let bindings = bindgen::builder()
		.header("cups.h")
		// Allowlist: types
		.allowlist_type("cups_dest_t")
		.allowlist_type("cups_option_t")
		// Allowlist: functions
		.allowlist_function("cupsGetDests")
		.allowlist_function("cupsFreeDests")
		.generate()
		.expect("Unable to generate bindings for CUPS");
	
	bindings
		.write_to_file(out_dir.join("cups-bindings.rs"))
		.expect("Unable to write bindings for CUPS");
}
