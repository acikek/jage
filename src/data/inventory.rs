extern crate serde;

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::common::Named;
use super::data::GameData;
use super::entity::Player;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum StatusEffectType {
    Health(isize),
    Enhance(HashMap<String, isize>)
}

impl StatusEffectType {
    pub fn apply(&self, player: &mut Player) {
        use StatusEffectType::*;

        match &self {
            Health(n) => { player.vitality.health += *n },
            Enhance(_) => ()
        }
    }

    pub fn apply_all(v: &Vec<StatusEffectType>, player: &mut Player) {
        for e in v {
            e.apply(player);
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct StatusEffect {
    pub name: String,
    pub description: String,
    pub cycle: Vec<StatusEffectType>
}

impl Named for StatusEffect {
    fn name(&self) -> String {
        self.name.clone()
    }
}

impl StatusEffect {
    pub fn cycle(&self, player: &mut Player) {
        StatusEffectType::apply_all(&self.cycle, player);
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct Consumable {
    pub restore: usize,
    pub effect: HashMap<String, usize>
}

#[derive(Debug, Deserialize, Clone)]
pub struct Equippable {
    pub strength: usize,
    #[serde(alias = "prof")]
    pub proficiency: Vec<String>
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum ItemType {
    Consumable(Consumable),
    Weapon(Equippable),
    Armor(Equippable),
    Material,
    Special,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Item {
    pub name: String,
    pub description: String,
    #[serde(rename = "type")]
    pub i_type: ItemType
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
pub struct Equipped {
    pub weapon: Option<String>,
    pub armor: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Inventory {
    pub items: HashMap<String, usize>,
    pub currency: Currency,
    pub equipped: Equipped
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

    pub fn display(&self, game: &GameData) -> Option<Vec<String>> {
        if self.items.is_empty() {
            return None;
        }

        Some(
            self.items.iter()
                .map(|(i, v)| {
                    format!("{} {}", v, game.items.get(i).unwrap().name)
                })
                .collect::<Vec<String>>()
        )
    }

    pub fn display_line(&self, game: &GameData) -> String {
        match self.display(game) {
            Some(v) => v.join(", "),
            None => String::from("Nothing")
        }
    }

    pub fn display_list(&self, game: &GameData) -> String {
        match self.display(game) {
            Some(v) => {
                v.iter()
                    .map(|i| format!("- {}", i))
                    .collect::<Vec<String>>()
                    .join("\n")
            }
            None => String::from("Nothing")
        }
    }

    pub fn drop(&self, other: &mut Inventory) {
        other.currency.value += self.currency.value;

        for (i, v) in &self.items {
            *other.items.get_mut(i).unwrap() += v;
        }
    }
}