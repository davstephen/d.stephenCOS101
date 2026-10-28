fn main(){
	let v = vec![101, 250, 330, 400];
	//vector v owns the object in the heap

	//ony a single variable owns the heap memory at any given time
	let v2 = v.clone();
	//here 2 variables owns heap value,
	//two pointers to the same content is not allowed in rust

	//Rust is very smart in terms of memory access, so it detects a race condition 
	//as two varialbles point to the same heap

	println!("{:?}",v );
}