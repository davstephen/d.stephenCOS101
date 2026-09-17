// Rust program to calculate the roots of quadratic equations
use std::io;

fn main() {
    //listing out all my inputs
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();
// Input a
    println!("\nEnter your first value (a)");
    io::stdin().read_line(&mut input1);
    let a:f32 = input1.trim().parse().expect("Not a valid number");
// Input b
    println!("\nEnter your second value (b)");
    io::stdin().read_line(&mut input2);
    let b:f32 = input2.trim().parse().expect("Not a valid number");
// Input c
    println!("\nEnter your third value (c)");
    io::stdin().read_line(&mut input3);
    let c:f32 = input3.trim().parse().expect("Not a valid number");

// Actual calculation
   let d = b * b - 4.0 * a * c;
   let root1 = (-b + d.sqrt()) / 2.0 * a;
   let root2 = (-b - d.sqrt()) / 2.0 * a;
 
 //Nature of the result (d) and the actual roots
 if d > 0.0 {
    println!("There are two distinct roots: ");
    println!("Root 1= {}",root1);
    println!("Root 2= {}",root2);

 } 
 else if d == 0.0 {
    println!("There is exactly one real root: ");
    println!("Root= {}",root1);

 }
 else{
    println!("The equation has no real roots (as the discriminant is less than 0) d: {}",d);
 }
}