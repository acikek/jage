extern crate rand;
extern crate serde;

use std::collections::HashMap;

use rand::distributions::{WeightedIndex, Distribution};
use serde::{Deserialize, Serialize};

use super::inventory::Inventory;
use super::common::{InteractionType, Range};

#[derive(Debug, Deserialize, Clone)]
pub struct EntityAttack {
    pub name: String,
    pub strength: usize,
    pub weight: u8,
    pub effects: Option<HashMap<String, usize>>
}

impl EntityAttack {
    pub fn apply(&self, player: &mut Player) {
        player.vitality.health -= self.strength;

        if let Some(m) = &self.effects {
            for (e, n) in m.clone() {
                if player.vitality.effects.contains_key(&e) {
                    *player.vitality.effects.get_mut(&e).unwrap() += n;
                } else {
                    player.vitality.effects.insert(e.clone(), n);
                }
            }
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Entity {
    pub health: usize,
    pub name: String,
    pub inventory: Inventory,
    pub attacks: Vec<EntityAttack>
}

impl Entity {
    pub fn choose_attack(&self) -> &EntityAttack {
        let weights = self.attacks.iter()
            .map(|a| a.weight)
            .collect::<Vec<u8>>();

        let dist = WeightedIndex::new(&weights).unwrap();
        
        &self.attacks[dist.sample(&mut rand::thread_rng())]
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct EntityInstance {
    pub base: String,
    pub health: Range
}

impl EntityInstance {
    pub fn from(e: &Entity, base: String) -> Self {
        EntityInstance {
            base,
            health: Range::new(e.health)
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct Character {
    pub name: String,
    pub description: String,
    pub interaction: InteractionType
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PlayerCombatData {
    pub entities: Vec<EntityInstance>
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum PlayerStatus {
    Combat(PlayerCombatData),
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
pub struct PlayerVitality {
    pub health: Range,
    pub effects: HashMap<String, usize>
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Player {
    pub name: String,
    pub class: String,
    pub skills: Vec<String>,
    pub vitality: PlayerVitality,
    pub status: PlayerStatus,
    pub location: String,
    pub inventory: Inventory,
    pub quests: PlayerQuestData,
    pub stats: PlayerStats
}