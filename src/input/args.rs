#[derive(Debug)]
pub struct Args {
    pub command: String,
    pub list: Vec<String>,
    pub input: String
}

impl Args {
    pub fn parse(line: &str) -> Self {
        let a: Vec<&str> = line.split(" ").collect();

        let command = String::from(a[0]).to_lowercase();
        let list: Vec<String> = (&a[1..])
            .to_vec()
            .iter()
            .map(|s| String::from(*s))
            .filter(|s| !s.is_empty())
            .collect();

        let input = list.join(" ");

        Self {
            command,
            list,
            input
        }
    }

    pub fn check(&self, min: usize) -> bool {
        self.list.len() >= min
    }

    pub fn check_err(&self, min: usize) -> Result<(), String> {    
        if !self.check(min) { 
            Err(String::from("You didn't supply enough arguments for that."))
        } else {
            Ok(())
        }
    }

    pub fn input_from(&self, offset: usize) -> String {
        self.list[offset..].join(" ")
    }
}