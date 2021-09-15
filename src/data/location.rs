extern crate rand;
extern crate serde;

use rand::seq::SliceRandom;
use serde::Deserialize;

use std::collections::HashMap;

use super::common::{InteractionLine, Named};
use super::data::GameData;
use super::entity::Character;
use super::inventory::Currency;
use super::time::GameTime;
use super::super::input::controller::InputController;

#[derive(Debug, Deserialize, Clone)]
pub struct Coordinates {
    pub x: usize,
    pub y: usize
}

impl Coordinates {
    pub fn as_float(&self) -> (f64, f64) {
        (self.x as f64, self.y as f64)
    }

    pub fn dist(&self, c: &Coordinates) -> f64 {
        let f1 = self.as_float();
        let f2 =    c.as_float();

        ((f2.1 - f1.1) + (f2.0 - f1.0)).abs().sqrt()
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct Citizen {
    pub name: String,
    #[serde(alias = "lines")]
    pub dialogue: Vec<String>
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum HouseResident {
    Citizen(Citizen),
    Character(Character)
}

#[derive(Debug, Deserialize, Clone)]
pub struct House {
    pub name: String,
    pub entry: Vec<InteractionLine>,
    pub residents: Vec<HouseResident>
}

impl Named for House {
    fn name(&self) -> String {
        self.name.clone()
    }
}

impl House {
    pub fn list(&self) -> Vec<String> {
        use HouseResident::*;

        self.residents.iter()
            .map(|r| match r {
                Citizen(c) => c.name.clone(),
                Character(c) => c.name.clone()
            })
            .collect::<Vec<String>>()
    }

    pub fn talk(&self, resident: usize, game: &mut GameData, input: &mut InputController) -> Option<String> {
        use HouseResident::*;

        match &self.residents[resident] {
            Citizen(c) => match c.dialogue.choose(&mut rand::thread_rng()) {
                Some(s) => Some(s.clone()),
                None => None
            },
            Character(c) => c.interaction.interact(game, input)
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct Inn {
    cost: usize
}

#[derive(Debug, Deserialize, Clone)]
pub struct Tavern {
    name: String,
    shop: Vec<String>,
    quests: Vec<String>
}

#[derive(Debug, Deserialize, Clone)]
pub struct Shop {
    name: String,
    items: Vec<String>
}

#[derive(Debug, Deserialize, Clone)]
pub struct Palace {
    name: String,
    ruler: Character
}

#[derive(Debug, Deserialize, Clone)]
pub struct TownData {
    houses: Vec<String>,
    inn: Inn,
    tavern: Tavern
}

#[derive(Debug, Deserialize, Clone)]
pub struct CityData {
    pub houses: Vec<String>,
    pub inn: Inn,
    pub tavern: Tavern,
    pub shops: Vec<Shop>
}

#[derive(Debug, Deserialize, Clone)]
pub struct CapitalData {
    pub houses: Vec<String>,
    pub inn: Inn,
    pub tavern: Tavern,
    pub shops: Vec<Shop>,
    pub palace: Palace
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum LocationType {
    Town(TownData),
    City(CityData),
    Capital(CapitalData),
    Bandit,
    Monument,
    Dungeon,
    Trader
}

#[derive(Debug, Deserialize, Clone)]
pub struct Location {
    #[serde(alias = "pos")]
    pub position: Coordinates,
    pub name: String,
    pub description: String,
    #[serde(alias = "reputation")]
    pub entry: Option<usize>,
    #[serde(rename = "type")]
    pub l_type: LocationType,
    pub quests: Option<Vec<String>>
}

impl Named for Location {
    fn name(&self) -> String {
        self.name.clone()
    }
}

pub enum ReputationLevel {
    Lowest,
    Low,
    Medium,
    High,
    Highest
}

impl ReputationLevel {
    pub fn name(l: ReputationLevel) -> &'static str {
        use ReputationLevel::*;

        match l {
            Lowest => "hostile",
            Low => "suspicious",
            Medium => "indifferent",
            High => "esteemed",
            Highest => "benevolent"
        }
    }

    pub fn value(rep: usize) -> Self {
        use ReputationLevel::*;

        match rep {
            0..=9 => Lowest,
            10..=24 => Low,
            25..=59 => Medium,
            60..=84 => High,
            85..=100 => Highest,
            _ => Medium
        }
    }
}

impl Location {
    pub fn reputation(&self, id: &String, reputation: &HashMap<String, usize>) -> (usize, bool) {
        if !reputation.contains_key(id) {
            let value = match self.entry {
                Some(n) => n,
                None => 50
            };

            //reputation.insert(id.clone(), value);
            (value, true)
        } else {
            (*reputation.get(id).unwrap(), false)
        }
    }

    pub fn entry(&self, id: &String, reputation: &HashMap<String, usize>) -> (String, usize, bool) {
        let rep = self.reputation(&id, reputation);

        let first = format!("You're entering {}{}", 
            self.name,
            if rep.1 { " for the first time." } else { "." }
        );

        let second = format!("Your reputation here is {} ({}).",
            rep.0,
            ReputationLevel::name(ReputationLevel::value(rep.0))
        );

        (format!("{} {}", first, second), rep.0, rep.1)
    }

    pub fn travel_prompt(&self, next: &Location, game: &GameData) -> (String, f64, usize) {
        let dist = self.position.dist(&next.position);
        let cost = dist * game.config.world.currency.dist_cost as f64;
        let time = (dist * 20.0) as usize;

        let prompt = format!("You are traveling from {} to {}.\n\nYou can ride the carriage, costing {}, or...\nYou can walk, taking {}.",
            self.name,
            next.name,
            Currency::display(cost, game),
            GameTime::duration(time)
        );

        (prompt, cost, time)
    }

    pub fn houses(&self, game: &GameData) -> Option<Vec<(String, String)>> {
        use LocationType::*;

        let houses = match &self.l_type {
            Town(t) => &t.houses,
            City(c) => &c.houses,
            Capital(c) => &c.houses,
            _ => return None
        };

        Some(houses.iter()
            .map(|h| {
                let house = game.houses.get(h).unwrap();
                (h.clone(), house.name.clone())
            })
            .collect::<Vec<(String, String)>>())
    }  
}