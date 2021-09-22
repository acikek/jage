extern crate serde;

use std::collections::HashMap;
use std::fmt::Display;
use std::ops::{AddAssign, SubAssign};

use serde::{Deserialize, Serialize};

use super::super::input::controller::InputController;

use super::data::GameData;
use super::entity::{Player, PlayerStatus, PlayerCombatData, EntityInstance};

pub trait Named {
    fn name(&self) -> String;
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Range {
    pub max: usize,
    pub value: usize
}   

impl Range {
    pub fn new(n: usize) -> Self {
        Range { 
            max: n,
            value: n
        }
    }

    pub fn set(&mut self, value: isize) {
        if value > 0 {
            self.value += value as usize;
        } else {
            if value.abs() as usize >= self.value {
                self.value = 0;
            } else {
                self.value -= value.abs() as usize;
            }
        }
    }
}

impl AddAssign<usize> for Range {
    fn add_assign(&mut self, value: usize) {
        self.set(value as isize);
    }
}

impl AddAssign<isize> for Range {
    fn add_assign(&mut self, value: isize) {
        self.set(value);
    }
}

impl SubAssign<usize> for Range {
    fn sub_assign(&mut self, value: usize) {
        self.set(-(value as isize));
    }
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum InteractionLine {
    Action(String),
    Dialogue(String)
}

impl Display for InteractionLine {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        use InteractionLine::*;

        write!(fmt, "{}", match self {
            Action(s) => s.clone(),
            Dialogue(s) => format!("\"{}\"", s)
        })
    }
}

impl InteractionLine {
    pub fn all(v: &Vec<InteractionLine>) -> String {
        v.iter()
            .map(|t| format!("{}", t))
            .collect::<Vec<String>>()
            .join("\n\n")
    }
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum Reward {
    Currency(f64),
    Items(HashMap<String, isize>),
    Quests(Vec<String>),
    Marks(Vec<String>),
    Logs(Vec<String>),
    Complete(Vec<String>)
}

pub fn apply_vec(v: &Vec<String>, p: &mut Option<Vec<String>>) {
    match p {
        Some(items) => {
            for d in v {
                if !items.contains(d) {
                    items.push(d.clone());
                }
            }
        }
        None => {
            *p = Some(v.clone());
        }
    }    
}

impl Reward {
    pub fn apply(&self, player: &mut Player) {
        use Reward::*;

        match self {
            Currency(n) => { 
                let _ = player.inventory.currency.add(*n, None); 
            }
            Items(m) => {
                for i in m {
                    player.inventory.add(i.0, *i.1);
                }
            }
            Quests(v) => apply_vec(v, &mut player.quests.assigned),
            Marks(v) => apply_vec(v, &mut player.stats.marks),
            Logs(v) => apply_vec(v, &mut player.stats.log),
            Complete(v) => {
                for q in v {
                    player.quests.complete(q);
                }
            }
        }
    }

    pub fn apply_all(v: &Vec<Reward>, player: &mut Player) {
        for r in v {
            r.apply(player);
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct StaticInteraction {
    pub lines: Vec<InteractionLine>,
    pub rewards: Option<Vec<Reward>>
}

#[derive(Debug, Deserialize, Clone)]
pub struct DynamicInteraction {
    pub lines: Vec<InteractionLine>,
    pub choices: Vec<DynamicChoice>
}

#[derive(Debug, Deserialize, Clone)]
pub struct CombatInteraction {
    pub lines: Vec<InteractionLine>,
    pub engage: HashMap<String, usize>
}

#[derive(Debug, Deserialize, Clone)]
pub struct DynamicChoice {
    pub response: String,
    pub interaction: InteractionType,
    pub conditions: Option<Vec<Condition>>
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum InteractionType {
    Static(StaticInteraction),
    Dynamic(DynamicInteraction),
    Combat(CombatInteraction)
}

impl InteractionType {
    pub fn interact(&self, game: &mut GameData, input: &mut InputController) -> Option<String> {
        use InteractionType::*;

        match self {
            Static(i) => {
                if let Some(r) = &i.rewards {
                    Reward::apply_all(r, &mut game.global.player);
                }

                Some(InteractionLine::all(&i.lines))
            }
            Dynamic(i) => {
                // Since choices can be filtered, we need to make a list of valid ones.
                // If we don't do this, the player will be choosing from an incomplete list.
                let choices = i.choices.iter()
                    .filter(|c| {
                        match &c.conditions {
                            Some(ls) => Condition::check_all(&ls, &game),
                            None => true
                        }
                    })
                    .collect::<Vec<&DynamicChoice>>();
                
                // Then, we can build the responses over our new choices.
                let responses = choices.iter()
                    .map(|d| d.response.clone())
                    .collect::<Vec<String>>();

                println!("{}", InteractionLine::all(&i.lines));

                let response = input.choice("What will you say?", responses, "You left the conversation.");

                match response {
                    Some(d) => {
                        let choice = &choices[d.1];
                        choice.interaction.interact(game, input)
                    }
                    None => None
                }
            }
            Combat(i) => {
                let mut entities: Vec<EntityInstance> = Vec::new();

                for (e, n) in &i.engage {
                    let instance = EntityInstance::from(game.entities.get(e).unwrap(), e.clone());

                    for _ in 0..*n {
                        entities.push(instance.clone());
                    }
                }

                let data = PlayerCombatData {
                    entities
                };

                game.global.player.status = PlayerStatus::Combat(data);
                
                Some(InteractionLine::all(&i.lines))
            }
        }
    }
}

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
    let vec = match o {
        Some(v) => v.clone(),
        None => Vec::new()
    };

    m.iter().all(|(k, v)| vec.contains(k) == *v)
}

pub fn display_quest_map(m: &HashMap<String, bool>, header: &str, game: &GameData) -> String {
    format!("{} {}", header, m.iter()
        .map(|(k, v)| {
            let quest = game.quests.get(k).unwrap();

            format!("{}{}", 
                if *v { String::new() } else { String::from("NOT ") },
                quest.name
            )
        })
        .collect::<Vec<String>>()
        .join(", ")
    )
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
    Finished(HashMap<String, bool>),
    Assigned(HashMap<String, bool>),
    Marks(HashMap<String, bool>)
}

impl Condition {
    pub fn check(&self, game: &GameData) -> bool {
        use Condition::*;

        let player = &game.global.player;

        match self {
            Currency(n) => player.inventory.currency.value >= *n,
            Health(n) => player.vitality.health.value >= *n,
            Items(m) => compare_map(m, &player.inventory.items),
            Reputation(m) => compare_map(m, &player.stats.reputation),
            Defeated(m) => compare_map(m, &player.stats.defeated),
            Completed(m) => compare_map_bool(&m, player.quests.completed.as_ref()),
            Finished(m) => compare_map_bool(&m, game.finished_quests().as_ref()),
            Assigned(m) => compare_map_bool(&m, player.quests.assigned.as_ref()),
            Marks(m) => compare_map_bool(&m, player.stats.marks.as_ref())
        }
    }

    pub fn check_all(v: &Vec<Condition>, game: &GameData) -> bool {
        v.iter()
            .all(|c| c.check(game))
    }

    pub fn display(&self, game: &GameData) -> String {
        use Condition::*;

        match self {
            Currency(n) => format!("Have at least {}", super::inventory::Currency::display(*n, game)),
            Health(n) => format!("Have at least {} health", n),
            Items(m) => {
                format!("Have {}", m.iter()
                    .map(|(k, v)| {
                        let item = game.items.get(k).unwrap();
                        format!("{} {}", v, item.name.clone())
                    })
                    .collect::<Vec<String>>()
                    .join(", "))
            },
            Reputation(m) => {
                format!("Have reputation {}", m.iter()
                    .map(|(k, v)| {
                        let loc = game.locations.get(k).unwrap();
                        format!("{} in {}", v, loc.name)
                    })
                    .collect::<Vec<String>>()
                    .join(", "))
            },
            Defeated(_) => {
                format!("Defeat (todo)")
            },
            Completed(m) => display_quest_map(m, "Complete", game),
            Finished(m) => display_quest_map(m, "Finish", game),
            Assigned(m) => display_quest_map(m, "Be assigned", game),
            Marks(_) => String::new()
        }
    }

    pub fn display_all(v: &Vec<Condition>, game: &GameData) -> Vec<String> {
        v.iter()
            .map(|c| c.display(game))
            .collect::<Vec<String>>()
    }
}