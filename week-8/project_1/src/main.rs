use std::io;

// Input
fn read_line() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().to_string()
}

fn main() {
    // Vectors for the APS
    let levels: Vec<&str> = vec!["APS 1-2", "APS 3-5", "APS 5-8", "EL1 8-10", "EL2 10-13", "SES"];
    let min_years: Vec<u32> = vec![1, 3, 5, 8, 10, 13];
    let max_years: Vec<u32> = vec![2, 5, 8, 10, 13, u32::MAX]; // SES has no upper limit

    // Vector for professions
    let professions: Vec<&str> = vec!["Office Admin", "Academic", "Lawyer", "Teacher"];


    let roles: Vec<Vec<&str>> = vec![
        vec!["Intern", "Administrator", "Senior Administrator", "Office Manager", "Director", "CEO"],
        vec!["-", "Research Assistant", "PhD Candidate", "Post-Doc Researcher", "Senior Lecturer", "Dean"],
        vec!["Paralegal", "Junior Associate", "Associate", "Senior Associate 1-2", "Senior Associate 3-4", "Partner"],
        vec!["Placement", "Classroom Teacher", "Senior Teacher", "Leading Teacher", "Deputy Principal", "Principal"],
    ];

    println!("=== Federal Government of Nigeria: APS Level Checker ===\n");

    // Choice
    println!("Choose a profession:");
    for i in 0..professions.len() {
        println!("  {}. {}", i + 1, professions[i]);
    }
    let p_choice: usize = match read_line().parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Invalid input. Enter a number.");
            return;
        }
    };
    
    if p_choice < 1 || p_choice > professions.len() {
        println!("Invalid profession number.");
        return;
    }
    let p_index = p_choice - 1;

    // role chioce specifically
    println!("\nChoose a role for {}:", professions[p_index]);
    for i in 0..roles[p_index].len() {
        if roles[p_index][i] != "-" {
            println!("  {}. {}", i + 1, roles[p_index][i]);
        }
    }
    let r_choice: usize = match read_line().parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Invalid input. Enter a number.");
            return;
        }
    };
    
    let role = match roles[p_index].get(r_choice.wrapping_sub(1)) {
        Some(r) if *r != "-" => *r,
        _ => {
            println!("Invalid role number.");
            return;
        }
    };
    let level_index = r_choice - 1;

    // Experience
    println!("\nEnter years of experience:");
    let years: u32 = match read_line().parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Invalid input. Enter a whole number.");
            return;
        }
    };

    // Confirmation
    println!("\n--- Result ---");
    println!("Profession : {}", professions[p_index]);
    println!("Role       : {}", role);
    println!("Level      : {}", levels[level_index]);

    if years >= min_years[level_index] && years <= max_years[level_index] {
        println!("Valid: {} years of experience fits {}.", years, levels[level_index]);
    } else {
        println!(
            "Not valid: {} years does not fit {} (expected {}{}).",
            years,
            levels[level_index],
            min_years[level_index],
            if max_years[level_index] == u32::MAX {
                "+ years".to_string()
            } else {
                format!(" to {} years", max_years[level_index])
            }
        );
       
        for i in 0..levels.len() {
            if years >= min_years[i] && years <= max_years[i] {
                println!("Hint: {} years matches {} ({}).", years, levels[i], roles[p_index][i]);
                break;
            }
        }
    }
}