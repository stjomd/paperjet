use std::{env, path};

fn main() {
	let target_family = env::var("CARGO_CFG_TARGET_FAMILY").unwrap();

	if target_family == "unix" {
		// Link CUPS
		println!("cargo:rustc-link-lib=cups");
		// Generate bindings
		cups_bindings();
	}
}

const CUPS_ALLOWED_FUNCTIONS: &[&str] = &[
	"cupsAddOption",
	"cupsCopyDestInfo",
	"cupsCreateDestJob",
	"cupsFinishDestDocument",
	"cupsFreeDests",
	"cupsGetDests",
	"cupsLastErrorString",
	"cupsStartDestDocument",
	"cupsWriteRequestData",
];
fn cups_bindings() {
	let mut builder = bindgen::builder().header("headers/cups.h");

	// Allowlist:
	for function in CUPS_ALLOWED_FUNCTIONS {
		builder = builder.allowlist_function(function);
	}
	// Type config:
	builder = builder
		.newtype_enum("ipp_status_e")
		.newtype_enum("http_status_e");

	// Generate & write:
	let out_dir = path::PathBuf::from(env::var("OUT_DIR").unwrap());
	let bindings = builder
		.generate()
		.expect("Unable to generate bindings for CUPS");
	bindings
		.write_to_file(out_dir.join("cups-bindings.rs"))
		.expect("Unable to write bindings for CUPS");
}
