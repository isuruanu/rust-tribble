fn basic_test() {
	let x = &5;

	match x {
		&val => println!("Value {}, val {}", x, val)
	}
}

fn ref_test() {
	let x = 5;

	match x {
		ref r => println!("Got reference {}", r)
	}
}

fn ref_mut_test() {
	let mut x = 5;
	match x {
		ref mut x => println!("Value {}", x)
	}
}

fn main() {
	basic_test();

	ref_test();

	ref_mut_test();
}
