use crate::cli::args::PrintArgs;
use printrs::print_files;

/// The `print` command
pub fn print(args: PrintArgs) {
	let result = print_files(&args.files);
	match result {
		Ok(_) => println!("File has been submitted for printing."),
		Err(e) => eprintln!("{e}"),
	}
}
