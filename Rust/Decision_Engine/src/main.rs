mod engine;

use std::ascii::AsciiExt;
use engine::DecisionEngine;
use std::io;
use std::io::Write;

fn main() {
    let mut decision_engine = DecisionEngine::create_engine();
    println!("You should use: {}", decision_engine.analyze());
    let mut again = String::from("");
    print!("Would you like to take the Programming Languages quiz again? Answer yes or no, or y or n: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut again).expect("Could not read line from agian prompt, main.rs Line 11");
    again = again.trim().to_string();

    while again.to_ascii_lowercase() == "yes" || again.to_ascii_lowercase() == "y" {
        decision_engine.ask_questions();
        println!("You should use: {}", decision_engine.analyze());
        again.clear();
        print!("Would you like to take the Programming Languages quiz again? Answer yes or no, or y or n: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut again).expect("Could not read line from agian prompt, main.rs Line 11");
        again = again.trim().to_string();
    }
}
