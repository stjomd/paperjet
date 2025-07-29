use printrs::print_file;

use crate::cli::args::PrintArgs;

/// The `print` command
pub fn print(_: PrintArgs) {
	print_file();
}
