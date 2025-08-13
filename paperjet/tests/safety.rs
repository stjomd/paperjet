#[test]
fn compilation_tests() {
	let t = trybuild::TestCases::new();
	t.compile_fail("tests/trybuild/dest_lifetimes.rs");
}
