extern crate serde;

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ItemType {
    Consumable(usize),
    Weapon(usize),
    Armor(usize),
    Special,
}

#[derive(Debug, Deserialize)]
pub struct Item {
    name: String,
    description: String,
    #[serde(rename = "type")]
    i_type: ItemType,
    #[serde(alias = "prof")]
    proficiency: Vec<String>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Inventory {
    pub items: HashMap<String, usize>,
    pub currency: usize
}

impl Inventory {
    pub fn get(&self, item: &String) -> Option<&usize> {
        self.items.get(item)
    }
}