struct Point {
	x : i32,
	y : i32,
	z : i32
}

fn basic_test(p : Point) {
	match p {
		Point{ y, x, ..} => println!("x value = {}, y value = {}", x, y)
	}
}

fn main() {
	basic_test(Point{x : 5, y : 10, z : 100});
}
