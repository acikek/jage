extern crate serde;

use std::collections::HashMap;
use std::ops::{AddAssign, SubAssign};

use serde::{Deserialize, Serialize};

use super::super::input::controller::InputController;
use super::super::input::args::Args;

use super::data::GameData;
use super::location::House;
use super::inventory::Inventory;
use super::quest::Condition;

#[derive(Debug, Deserialize, Serialize)]
pub struct Range {
    pub max: usize,
    pub value: usize
}

impl Range {
    pub fn set(&mut self, value: usize) {
        if value <= self.max {
            self.value = value;
        } else {
            self.value = self.max;
        }
    }
}

impl AddAssign<usize> for Range {
    fn add_assign(&mut self, value: usize) {
        self.set(self.value + value);
    }
}

impl SubAssign<usize> for Range {
    fn sub_assign(&mut self, value: usize) {
        self.set(self.value - value);
    }
}

#[derive(Debug, Deserialize)]
pub struct Entity {
    health: usize,
    name: String,
    drops: Vec<String>
}

#[derive(Debug, Deserialize)]
pub struct StaticInteraction {
    pub line: String
}

#[derive(Debug, Deserialize)]
pub struct DynamicInteraction {
    pub line: String,
    pub choices: Vec<DynamicChoice>
}

#[derive(Debug, Deserialize)]
pub struct DynamicChoice {
    pub response: String,
    pub interaction: InteractionType,
    pub conditions: Option<Vec<Condition>>
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InteractionType {
    Static(StaticInteraction),
    Dynamic(DynamicInteraction)
}

impl InteractionType {
    pub fn interact(&self, game: &GameData, input: &mut InputController) -> Option<String> {
        use InteractionType::*;

        match self {
            Static(i) => Some(i.line.clone()),
            Dynamic(i) => {
                // Since choices can be filtered, we need to make a list of valid ones.
                // If we don't do this, the player will be choosing from an incomplete list.
                let choices = i.choices.iter()
                    .filter(|c| {
                        match &c.conditions {
                            Some(ls) => Condition::check_list(&ls, &game.global.player),
                            None => true
                        }
                    })
                    .collect::<Vec<&DynamicChoice>>();
                
                // Then, we can build the responses over our new choices.
                let responses = choices.iter()
                    .map(|d| d.response.clone())
                    .collect::<Vec<String>>();

                println!("\"{}\"", i.line);

                let response = input.choice("What will you say?", responses, "You left the conversation.");

                match response {
                    Some(d) => {
                        let choice = &choices[d.1];
                        choice.interaction.interact(game, input)
                    }
                    None => None
                }
            }
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Character {
    pub name: String,
    pub description: String,
    pub interaction: InteractionType
    //pub quests: Option<Vec<String>>
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PlayerStatus {
    Combat(HashMap<String, usize>),
    House(String),
    Location,
    #[serde(alias = "none")]
    Idle
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PlayerQuestData {
    pub assigned: Option<Vec<String>>,
    pub completed: Option<Vec<String>>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PlayerStats {
    pub reputation: HashMap<String, usize>,
    pub defeated: HashMap<String, usize>
}

#[derive(Debug, Deserialize, Serialize)]
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