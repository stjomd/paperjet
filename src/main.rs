mod print;

fn main() {
	let printers = print::get_printers();
	println!("{:#?}", printers);
}
