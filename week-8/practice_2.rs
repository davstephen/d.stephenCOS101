fn main() {

	let v = vec!['C', 'O', 'M', 'P', 'U', 'T',];

	let mut input1 = String::new();

	println!("Enter an index value btw (0 - 7)");
	std::io::stdin().read_line(&mut input1).expect("Failde to read input");
	//index is the non negatice value which is smaller than the size of the vector
	let index:usize = input1.trim().parse().expect("Invalid input");

	//getting vaur at given index value 
	let ch: char = v[index];


	print!("{} is the character for index [{}]\n",ch, index);
}