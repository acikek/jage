extern crate serde;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Skill {
    pub name: String,
    pub description: String
}

#[derive(Debug, Deserialize)]
pub struct Class {
    pub name: String,
    pub description: String,
    pub proficiency: Vec<String>
}
