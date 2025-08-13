fn main() {
	let _ = {
		let mut destinations = paperjet::unix::dest::CupsDestinations::new();
		destinations.into_iter().next().unwrap()
	};
}
