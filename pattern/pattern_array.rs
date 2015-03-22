fn basic_test() {
	let v = vec!["value", "1"];

	match &v[..] {
		["value", number] => println!("{} got it", number),
		_ => {}
	}
}

fn main() {
	basic_test();
}
