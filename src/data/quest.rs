extern crate serde;

use std::collections::HashMap;

use serde::Deserialize;

use super::entity::Player;

pub fn compare_map(m1: &HashMap<String, usize>, m2: &HashMap<String, usize>) -> bool {
    for item in m1 {
        if m2.contains_key(item.0) {
            if *m2.get(item.0).unwrap() < *item.1 {
                return false
            }
        } else {
            if item.1 <= &0 {
                continue;
            } else {
                return false
            }
        }
    }

    true
}

pub fn compare_map_bool(m: &HashMap<String, bool>, o: Option<&Vec<String>>) -> bool {
    match o {
        Some(vec) => {
            for item in m {
                if vec.contains(item.0) {
                    if *item.1 {
                        continue;
                    } else {
                        return false
                    }
                } else {
                    if !item.1 {
                        continue;
                    } else {
                        return false
                    }
                }
            }
        },
        None => {
            for item in m {
                if !item.1 {
                    continue;
                } else {
                    return false
                }
            }
        }
    }

    true
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum Condition {
    Currency(f64),
    Health(usize),
    Items(HashMap<String, usize>),
    Reputation(HashMap<String, usize>),
    Defeated(HashMap<String, usize>),
    Completed(HashMap<String, bool>),
    Assigned(HashMap<String, bool>)
}

impl Condition {
    pub fn check(&self, player: &Player) -> bool {
        use Condition::*;

        match self {
            Currency(n) => player.inventory.currency.value > *n,
            Health(n) => player.health.value > *n,
            Items(m) => compare_map(m, &player.inventory.items),
            Reputation(m) => compare_map(m, &player.stats.reputation),
            Defeated(m) => compare_map(m, &player.stats.defeated),
            Completed(m) => compare_map_bool(&m, player.quests.completed.as_ref()),
            Assigned(m) => compare_map_bool(&m, player.quests.assigned.as_ref())
        }
    }

    pub fn check_list(v: &Vec<Condition>, player: &Player) -> bool {
        for condition in v {
            let result = condition.check(player);

            if !result { 
                return false 
            }
        }

        true
    }
}

#[derive(Debug, Deserialize)]
pub struct Quest {
    pub name: String,
    #[serde(alias = "goal")]
    pub description: String,
    pub completion: String,
    pub requirements: Vec<Condition>
}

impl Quest {
    pub fn check(&self, player: &Player) -> bool {
        Condition::check_list(&self.requirements, player)
    }
}