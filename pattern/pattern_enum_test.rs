enum OptionalInt {
	Value(i32),
	Absent
}

fn basic_test(x : OptionalInt) {
	match x {
		OptionalInt::Value(..) => println!("Got int value"),
		OptionalInt::Absent => println!("No value")
	}
}

fn match_guard_test(x : OptionalInt) {
	match x {
		OptionalInt::Value(i) if i > 5 => println!("{} is greater than 5", i),
		OptionalInt::Value(j) => println!("has a value = {}", j),
		OptionalInt::Absent => println!("Absent value")
	}
}

fn main() {
	basic_test(OptionalInt::Value(5));
	basic_test(OptionalInt::Absent);

	match_guard_test(OptionalInt::Value(6));
	match_guard_test(OptionalInt::Value(4));
	match_guard_test(OptionalInt::Absent);
}
