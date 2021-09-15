extern crate serde;

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::inventory::Inventory;
use super::common::{InteractionType, Range};

#[derive(Debug, Deserialize)]
pub struct Entity {
    health: usize,
    name: String,
    drops: Vec<String>
}

#[derive(Debug, Deserialize, Clone)]
pub struct Character {
    pub name: String,
    pub description: String,
    pub interaction: InteractionType
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum PlayerStatus {
    Combat(HashMap<String, usize>),
    House(String),
    Location,
    #[serde(alias = "none")]
    Idle
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PlayerQuestData {
    pub assigned: Option<Vec<String>>,
    pub completed: Option<Vec<String>>
}

impl PlayerQuestData {
    pub fn remove(&mut self, q: &String) {
        match &mut self.assigned {
            Some(v) => {
                if v.contains(q) {
                    v.retain(|x| x != q);

                    if v.is_empty() {
                        self.assigned = None;
                    } 
                }
            }
            None => ()
        }
    }

    pub fn complete(&mut self, q: &String) {
        self.remove(q);

        match &mut self.completed {
            Some(v) => v.push(q.clone()),
            None => self.completed = Some(vec![q.clone()])
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PlayerStats {
    pub reputation: HashMap<String, usize>,
    pub defeated: HashMap<String, usize>,
    pub marks: Option<Vec<String>>,
    pub log: Option<Vec<String>>
}

impl PlayerStats {
    pub fn log(&self, last: bool) -> String {
        match &self.log {
            Some(v) => if last {
                v.last().unwrap().clone()
            } else { 
                v.join("\n\n")
            },
            None => String::from("Nothing here...")
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Player {
    pub name: String,
    pub class: String,
    pub skills: Vec<String>,
    pub health: Range,
    pub status: PlayerStatus,
    pub location: String,
    pub inventory: Inventory,
    pub quests: PlayerQuestData,
    pub stats: PlayerStats
}