use crate::cli::args::PrintArgs;
use printrs::print_file;

/// The `print` command
pub fn print(args: PrintArgs) {
	let result = print_file(args.input);
	match result {
		Ok(_) => println!("File has been submitted for printing."),
		Err(e) => eprintln!("{e}"),
	}
}
