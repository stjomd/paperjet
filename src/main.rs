use crate::args::ListArgs;

mod args;

fn main() {
	let args = args::parse();
	match args.command {
		args::Command::List(list_args) => list(list_args),
	}
}

fn list(_: ListArgs) {
	let printers = printrs::get_printers();
	println!("{:#?}", printers);
}
