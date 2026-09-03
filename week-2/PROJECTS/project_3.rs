fn main () {
	let p:f64 = 210000.0;
	let r:f64 = 5.0;
	let n:f64 = 3.0;

	//New Value 
	let d = p * ( 1.0 -(r / 100.0 )).powf(n);
	println!("The Value of the TV after 3 years is {}", d);

	//Depreciation
	let v = d - p;
	println!("The Depeciation on the TV is {}", v);
}