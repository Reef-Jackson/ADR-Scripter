use clearscreen::clear;
use std::io::stdin;

#[derive(Debug)]
pub struct ADR {
    pub title: String,
    pub context_problem_stmt: String,
    pub decision_drivers: Vec<String>,
    pub considered_options: Vec<String>,
    pub decision_outcome: String,
    pub chosen_because: String,
    pub consequences: Vec<String>,
}

impl ADR {
    pub fn new() -> Self {
        clear();
        let mut title = ADR::create_simple("Enter Title: ");
        let mut context_problem_stmt = ADR::create_simple("Enter Context Problem Statement: ");
        let mut decision_drivers = ADR::create_list("Enter decision driver [q/Q to finish]:");
        let mut considered_options = ADR::create_list("Enter considered options: [Q/q to finish]:");
        let mut decision_outcome = ADR::create_simple("Enter decision outcome:");
        let mut chosen_because = ADR::create_simple("Enter 'Chosen Because': ");
        let mut consequences = ADR::create_list("Enter consequences of change [q/Q to finish]");

        ADR {
            title: title,
            context_problem_stmt: context_problem_stmt,
            decision_drivers: decision_drivers,
            considered_options: considered_options,
            decision_outcome: decision_outcome,
            chosen_because: chosen_because,
            consequences: consequences,
        }
    }

    pub fn create_simple(prompt: &str) -> String {
        loop {
            println!("{}", prompt.green();
            let mut input_buf = String::new();
            stdin()
                .read_line(&mut input_buf)
                .expect("Failed to read line");

            let trimmed_input = input_buf.trim();

            if trimmed_input.is_empty() {
                println!("Please enter a value.".red());
                continue;
            }

            clear();

            return trimmed_input.to_string();
        }
    }

    pub fn create_list(prompt: &str) -> Vec<String> {
        let mut list = Vec::<String>::new();
        let mut exit = false;

        while !exit {
            let mut item = ADR::create_simple(prompt.clone().green());
            match item.as_str() {
                "Q" | "q" => {
                    exit = true;
                }
                _ => {
                    clear();
                    list.push(item.to_string());
                }
            };
        }

        list
    }
}
