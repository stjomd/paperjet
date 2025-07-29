use crate::cli::args::PrintArgs;
use printrs::print_file;

/// The `print` command
pub fn print(args: PrintArgs) {
	print_file(args.input);
}
