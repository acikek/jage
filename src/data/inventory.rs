extern crate serde;

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::data::GameData;

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
pub struct Currency {
    pub value: f64
}

impl Currency {
    pub fn take(&mut self, n: f64, plural: &String) -> Result<(), String> {
        if n > self.value {
            Err(format!("You don't have enough {}", plural))
        } else {
            self.value -= n;
            Ok(())
        }
    }

    pub fn display(n: f64, game: &GameData) -> String {
        format!("{:.2}{}", n, game.config.world.currency.symbol)
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Inventory {
    pub items: HashMap<String, usize>,
    pub currency: Currency
}

impl Inventory {
    pub fn get(&self, item: &String) -> Option<&usize> {
        self.items.get(item)
    }
}