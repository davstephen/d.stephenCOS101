// Rust program to determine the annual incentives of employees
use std::io;

fn main() {
    //input
    let mut input1 = String::new();
    let mut input2 = String::new();

     //Incentive amounts
    let tier1 = "N1,560,000";
    let tier2 = "N1,480,000";
    let tier3 = "N1,300,000";
    let tier4 = "N100,000";

    //Experienced choice
    println!("\nAre you an experienced employee? (true/false)");
    io::stdin().read_line(&mut input2);
    let exp:bool = input2.trim().parse().expect("Not a valid input, Input True/False");

    if exp == true{
    //Age input
    println!("\nPlease input your age: ");
    io::stdin().read_line(&mut input1);
    let age:i8 =input1.trim().parse().expect("Not a valid age");

    //Incentive Choice
    if exp == true {
        if age >= 40 {
            println!("Your annual incentive is: {}",tier1);
        }
        else if age >= 30 && age < 40 {
            println!("Your annual incentive is: {}",tier2);
        }
        else if age <28 {
            println!("Your annual incentive is: {}",tier3);
        }
    }
    }
    else {
        println!("Your annual incentive is: {}",tier4);
    }
}
