extern crate serde;

use serde::Deserialize;

use super::common::Named;

#[derive(Debug, Deserialize)]
pub struct Skill {
    pub name: String,
    pub description: String
}

impl Named for Skill {
    fn name(&self) -> String {
        self.name.clone()
    }
}

#[derive(Debug, Deserialize)]
pub struct Class {
    pub name: String,
    pub description: String,
    pub proficiency: Vec<String>
}


impl Named for Class {
    fn name(&self) -> String {
        self.name.clone()
    }
}