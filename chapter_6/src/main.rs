use rand::Rng; // Import the Rng trait from the rand crate
use chrono::Local;

fn main() {

    println!("Using the rand crate");


    let mut rng = rand::rng(); // Get a thread-local random number generator
    let anomaly_strength: u32 = rng.random_range(1..=10); // Generate a random number between 1 and 10

    println!("The quantum anomaly strength is: {}", anomaly_strength);


    println!("Using the chrono for time");

    let now = Local::now();
    println!("Starfleet log entry created at: {}", now.format("%Y-%m-%d %H:%M:%S").to_string());


}