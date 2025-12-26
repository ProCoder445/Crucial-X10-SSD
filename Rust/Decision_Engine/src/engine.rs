use std::io;
use std::io::Write;

pub struct DecisionEngine {
    simplicity: String,
    speed: String,
    efficiency: String,
}

impl DecisionEngine {
    pub fn create_engine() -> Self {
        let mut engine = Self {
            simplicity: String::new(),
            speed: String::new(),
            efficiency: String::new(),
        };

        engine.ask_questions();
        engine
    }

    pub fn ask_questions(&mut self) {
        self.simplicity.clear();
        print!("How simple do you want your language to be? (Simple / Moderate / Hard): ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut self.simplicity).unwrap();
        self.simplicity = self.simplicity.trim().to_ascii_lowercase();

        self.speed.clear();
        print!("How fast do you want to write programs? (Fast / Moderate / Slow): ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut self.speed).unwrap();
        self.speed = self.speed.trim().to_ascii_lowercase();

        self.efficiency.clear();
        print!("How efficient do you want your language to be? (Efficient / Moderate / No): ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut self.efficiency).unwrap();
        self.efficiency = self.efficiency.trim().to_ascii_lowercase();

        println!();
    }

    pub fn analyze(&self) -> String {
        let mut rust: i8 = 0;
        let mut go: i8 = 0;
        let mut python: i8= 0;

        match self.simplicity.as_str() {
            "simple" => python += 1,
            "moderate" => go += 1,
            "hard" => rust += 1,
            _ => {}
        }

        match self.speed.as_str() {
            "fast" => python += 1,
            "moderate" => go += 1,
            "slow" => rust += 1,
            _ => {}
        }

        match self.efficiency.as_str() {
            "efficient" => rust += 1,
            "moderate" => go += 1,
            "no" => python += 1,
            _ => {}
        }

        if rust > go && rust > python {
            "Rust".to_string()
        } else if go > rust && go > python {
            "Go".to_string()
        } else if python > go && python > rust {
            "Python".to_string()
        } else { "all three".to_string() }
    }
}
