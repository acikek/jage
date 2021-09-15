extern crate serde;

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::common::Named;
use super::data::GameData;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum ItemType {
    Consumable(usize),
    Weapon(usize),
    Armor(usize),
    Special,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Item {
    pub name: String,
    pub description: String,
    #[serde(rename = "type")]
    pub i_type: ItemType,
    #[serde(alias = "prof")]
    pub proficiency: Vec<String>
}

impl Named for Item {
    fn name(&self) -> String {
        self.name.clone()
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Currency {
    pub value: f64
}

impl Currency {
    pub fn add(&mut self, n: f64, plural: Option<&String>) -> Result<(), String> {
        if self.value + n < 0.0 {
            if plural.is_some() {
                Err(format!("You don't have enough {}", plural.unwrap()))
            } else {
                self.value = 0.0;
                Ok(())
            }
        } else {
            self.value += n;
            Ok(())
        }
    }

    pub fn display(n: f64, game: &GameData) -> String {
        format!("{:.2}{}", n, game.config.world.currency.symbol)
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Inventory {
    pub items: HashMap<String, usize>,
    pub currency: Currency
}

impl Inventory {
    pub fn add(&mut self, i: &String, n: isize) {
        if self.items.contains_key(i) {
            let item = self.items.get_mut(i).unwrap();
        
            if n > 0 {
                *item += n as usize;
            } else {
                if n.abs() as usize >= *item {
                    self.items.remove(i);
                } else {
                    *item -= n.abs() as usize;
                }
            }
        } else {
            if n > 0 {
                self.items.insert(i.clone(), n as usize);
            }
        }
    }

    pub fn get(&self, item: &String) -> Option<&usize> {
        self.items.get(item)
    }
}