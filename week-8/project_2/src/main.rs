use std::io;

// Keyboard input
fn read_line() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().to_string()
}

fn main() {
    println!("=== EY Nigeria: Most Experienced Developer ===\n");

    // vector
    let mut candidates: Vec<(String, u32)> = Vec::new();

    println!("How many candidates were interviewed?");
    let total: u32 = read_line().parse().expect("Invalid input");

    for i in 0..total {
        println!("\nCandidate {}", i + 1);

        println!("Name:");
        let name = read_line();

        println!("Years of programming experience:");
        let years: u32 = read_line().parse().expect("Invalid input");

        candidates.push((name, years));
    }

    if candidates.is_empty() {
        println!("\nNo candidates entered.");
        return;
    }

    //Filter
    let mut highest = candidates[0].1;
    for candidate in &candidates {
        if candidate.1 > highest {
            highest = candidate.1;
        }
    }

    // highest ig 
    println!("\n--- Interview Result ---");
    for (name, years) in &candidates {
        if *years == highest {
            println!("{} has the most experience: {} years", name, years);
        }
    }
}