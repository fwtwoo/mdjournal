use std::io;
use std::fs::File;
use std::io::Write;
use chrono::Local;

fn main() {
    // Print welcome message and questions
    println!("📓 Welcome to MDJournal!\n");

    // Get current date and format it
    let date = Local::now();
    let filename = date.format("%Y-%m-%d");

    // Run the questions function
    journal();

    // Create or open file
    let mut file = File::create(filename.to_string())
        .expect("Failed to create or open file");
    // Write to file
    file.write_all(b"Hello, Rust!")
        .expect("Failed to write to file");
    // Confirmation message
    println!("Data written to file successfully!");
}

fn journal() {
    // Creates new string object
    let mut input = String::new();

    // Create the questions
    let questions = [
        "1. What was the highlight of your day?",
        "2. Did you face any challenges today? How did you handle them?",
        "3. What is one thing you learned today?",
        "4. What are you grateful for today?",
        "5. What is one thing you want to improve on tomorrow?",
    ];

    // Vector of answers
    let mut answers: Vec<String> = Vec::new();

    // Print the questions
    for str in questions {
        println!("{}", str);

        // Read line
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        // Add answers to vector, trim to remove "/n",
        // and to_string() for correct type.
        answers.push(input.trim().to_string());
        input.clear();
    }
    // Debug print
    println!("{:?}", answers);
}
