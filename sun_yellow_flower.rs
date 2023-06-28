// This program will introduce users to the basics of rust and allow them to create their own rust program

// Include necessary modules
use std::io;

fn main() {
    println!("Welcome to Planting Seeds - let's get started learning rust!");
    println!();
    
    // Declare variables
    let mut input;
    let mut seed_type = String::new();
    let mut soil_type = String::new();
    let mut number_of_seeds = 0;
    let mut depth_of_seeds = 0;
    
    println!("What type of seed are you planting?");
    io::stdin()
        .read_line(&mut seed_type)
        .expect("Failed to read line");
    
    println!("What type of soil are you planting in?");
    io::stdin()
        .read_line(&mut soil_type)
        .expect("Failed to read line");
    
    println!("How many seeds do you want to plant?");
    input = io::stdin().read_line(&mut number_of_seeds).expect("Failed to read line");
    number_of_seeds = input.trim().parse().expect("Please type a number!");
    
    println!("How deep do you want to plant your seeds?");
    input = io::stdin().read_line(&mut depth_of_seeds).expect("Failed to read line");
    depth_of_seeds = input.trim().parse().expect("Please type a number!");
    
    // Output information to the user
    println!();
    println!("You are planting {} seeds in {} soil, planted at a depth of {} inches.", 
                number_of_seeds, soil_type.trim(), depth_of_seeds);
    
    // Print tips for successful planting
    println!("\nHere are a few tips for successful planting:");
    println!("- Make sure you water the soil adequately before planting");
    println!("- Follow any instructions given by seed packagers");
    println!("- Plant the seeds at the correct depth");
    println!("- Make sure you give the seedlings plenty of sunlight");
    println!("- Fertilize regularly to promote growth");
    println!("\nGood luck with your planting!");
}