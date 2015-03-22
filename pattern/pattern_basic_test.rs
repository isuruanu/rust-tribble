
fn simple_match(x : i32) {
	
	match x {
		1 => println!("one"),
		_ => println!("Not one")
	}
}

fn multiple_match(x : i32) {
	match x {
		1 | 2 => println!("one or two"),
		_ => println!("neither one or two")
	}
}

fn range_match(x : i32) {
	match x {
		1 ... 4 => println!("one through 4"),
		_ => println!("Not one through 4")
	}
}

fn value_bind_match(x : i32) {
	match x {
		e @ 1 ... 6 => println!("Value = {} is in the range 1-6", e),
		_ => println!("Value not in the range")
	}
}

fn main() {
	simple_match(1);

	multiple_match(2);
	
	range_match(1);

	value_bind_match(2);
}
