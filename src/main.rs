use chrono::Local;
use std::fs::File;
use std::io;
use std::io::Write;

// Global questions array
static QUESTIONS: [&str; 5] = [
    "🌟 What was the highlight of your day?",
    "🧗 Did you face any challenges today? How did you handle them?",
    "📚 What is one thing you learned today?",
    "🙏 What are you grateful for today?",
    "🎯 What is one thing you want to improve on tomorrow?",
];

fn main() {
    // Print welcome message and questions
    println!("📓 Welcome to MDJournal!\n");

    // Get current date and format it
    let date = Local::now();
    let filename = date.format("%Y-%m-%d");

    // Save the journal function as variable
    let answers = journal();
    // Format content using function
    let content = format(&answers);

    // Create or open file
    let mut file = File::create(filename.to_string()).expect("Failed to create or open file");

    // Combine and write to file
    let combined = content.join("\n");
    file.write_all(combined.as_bytes()) // Rust expects a byte slice
        .expect("Failed to write to file");

    // Confirmation message
    println!("\nData written to file successfully!");
}

// Function returns a Vector of strings
fn journal() -> Vec<String> {
    // Creates new string object
    let mut input = String::new();

    // Vector of answers
    let mut answers: Vec<String> = Vec::new();

    // Print the questions
    for q in QUESTIONS {
        println!("{}", q);

        // Read line
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        // Add answers to vector, trim to remove "/n",
        // and to_string() for correct type.
        answers.push(input.trim().to_string());
        input.clear();
    }
    // Return vector
    answers // Used later, in main()
}

// Takes answers as param, uses global QUESTIONS variable
fn format(answers: &[String]) -> Vec<String> {
    // Combines questions and answers using zip()
    QUESTIONS
        .iter()
        .zip(answers)
        .map(|(q, a)| format!("### {}\n{}\n", q, a))
        .collect()
}
