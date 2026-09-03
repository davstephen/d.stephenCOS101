fn main () {
	let t:f64 = 450000.0;
	let m:f64 = 1500000.0;
	let h:f64 = 750000.0;
	let d:f64 = 2850000.0;
	let a:f64 = 250000.0;

	//Sum of sales
	let s = (t * 2.0) + m + (h * 3.0) + (d * 3.0) + a;
	println!("Sum of sales {}", s);
	//average of sales
	let a = s / 10.0;
	println!("Average of Sales {}", a);
}