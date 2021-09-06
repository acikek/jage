extern crate rand;
extern crate serde;

use rand::seq::SliceRandom;
use serde::Deserialize;

use super::data::GameData;
use super::entity::Character;
use super::super::input::controller::InputController;

#[derive(Debug, Deserialize)]
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

#[derive(Debug, Deserialize)]
pub struct Citizen {
    pub name: String,
    #[serde(alias = "lines")]
    pub dialogue: Vec<String>
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HouseResident {
    Citizen(Citizen),
    Character(Character)
}

#[derive(Debug, Deserialize)]
pub struct House {
    residents: Vec<HouseResident>
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

    pub fn talk(&self, resident: usize, game: &GameData, input: &mut InputController) -> String {
        use HouseResident::*;

        match &self.residents[resident] {
            Citizen(c) => c.dialogue.choose(&mut rand::thread_rng()).unwrap().clone(),
            Character(c) => c.interaction.interact(game, input).unwrap()
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Inn {
    cost: usize
}

#[derive(Debug, Deserialize)]
pub struct Tavern {
    name: String,
    shop: Vec<String>,
    quests: Vec<String>
}

#[derive(Debug, Deserialize)]
pub struct Shop {
    name: String,
    items: Vec<String>
}

#[derive(Debug, Deserialize)]
pub struct Palace {
    name: String,
    ruler: Character
}

#[derive(Debug, Deserialize)]
pub struct TownData {
    houses: Vec<String>,
    inn: Inn,
    tavern: Tavern
}

#[derive(Debug, Deserialize)]
pub struct CityData {
    pub houses: Vec<String>,
    pub inn: Inn,
    pub tavern: Tavern,
    pub shops: Vec<Shop>
}

#[derive(Debug, Deserialize)]
pub struct CapitalData {
    pub houses: Vec<String>,
    pub inn: Inn,
    pub tavern: Tavern,
    pub shops: Vec<Shop>,
    pub palace: Palace
}

#[derive(Debug, Deserialize)]
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

#[derive(Debug, Deserialize)]
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
    pub fn reputation(id: &String, data: &mut GameData) -> (usize, bool) {
        if !data.global.player.stats.reputation.contains_key(id) {
            let value = match data.locations.get(id).unwrap().entry {
                Some(n) => n,
                None => 50
            };

            data.global.player.stats.reputation.insert(id.clone(), value);
            (value, true)
        } else {
            (*data.global.player.stats.reputation.get(id).unwrap(), false)
        }
    }

    pub fn entry(id: String, data: &mut GameData) -> String {
        let rep = Self::reputation(&id, data);

        let first = format!("You're entering {}{}", 
            data.locations.get(&id).unwrap().name,
            if rep.1 { " for the first time." } else { "." }
        );

        let second = format!("Your reputation here is {} ({}).",
            rep.0,
            ReputationLevel::name(ReputationLevel::value(rep.0))
        );

        format!("{} {}", first, second)
    }
}