extern crate rustyline;

use rustyline::Editor;

pub struct InputController {
    prompt: String,
    rl: Editor<()>
}

impl InputController {
    pub fn new(prompt: String) -> Self {
        Self {
            prompt,
            rl: Editor::<()>::new()
        }
    }

    pub fn read(&mut self) -> Option<String> {
        match self.rl.readline(self.prompt.as_str()) {
            Ok(s) => {
                self.rl.add_history_entry(&s);
                Some(s)
            },
            _ => None
        }
    }

    pub fn prompt(&mut self, text: String) -> Option<String> {
        println!("{}", text);
    
        self.read()
    }
    
    pub fn choice(&mut self, text: &str, options: Vec<String>) -> Option<(String, usize)> {
        let ops = options.iter()
            .enumerate()
            .map(|(i, o)| format!("{}) {}", i + 1, o))
            .collect::<Vec<String>>()
            .join("\n");

        println!("\n{}\n\n{}\n", text, ops);

        loop {
            match self.rl.readline(self.prompt.as_str()) {
                Ok(s) => {
                    match s.parse::<usize>() {
                        Ok(n) => {
                            match options.get(n - 1) {
                                Some(s) => return Some((s.clone(), n - 1)),
                                None => {
                                    println!("Value out of range.\n");
                                    continue;
                                }
                            }
                        }
                        Err(_) => {
                            println!("Invalid integer.\n");
                            continue;
                        }
                    }
                }
                _ => {
                    println!("You decided not to choose.");
                    return None
                }
            }
        }
    }
}

