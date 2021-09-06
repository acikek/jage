extern crate serde;

use serde::{Deserialize, Serialize};

use std::collections::HashMap;

use super::config::Config;
use super::entity::{Player, PlayerStatus};
use super::inventory::Item;
use super::location::{Location, House};
use super::quest::Quest;
use super::time::GameTime;
use super::super::fs::fs::Filesystem;

#[derive(Debug, Deserialize, Serialize)]
pub struct GlobalData {
    pub player: Player,
    pub time: GameTime
}

#[derive(Debug)]
pub struct GameData {
    pub config: Config,
    pub houses: HashMap<String, House>,
    pub items: HashMap<String, Item>,
    pub locations: HashMap<String, Location>,
    pub quests: HashMap<String, Quest>,
    pub global: GlobalData
}

impl GameData {
    pub fn from(fs: &Filesystem) -> Result<Self, Box<dyn std::error::Error>> {
        let config = Filesystem::parse(fs.read("jage.yml")?)?;
        let houses = Filesystem::parse_map(fs.read_dir("houses")?)?;
        let items = Filesystem::parse_map(fs.read_dir("items")?)?;
        let locations = Filesystem::parse_map(fs.read_dir("locations")?)?;
        let quests = Filesystem::parse_map(fs.read_dir("quests")?)?;
        let global = Filesystem::parse(fs.read("data/global.yml")?)?;

        Ok(Self {
            config,
            houses,
            items,
            locations,
            quests,
            global
        })
    }

    pub fn location(&self) -> &Location {
        self.locations.get(&self.global.player.location).unwrap()
    }

    pub fn house(&self) -> Option<&House> {
        if let PlayerStatus::House(h) = &self.global.player.status {
            Some(self.houses.get(h).unwrap())
        } else {
            None
        }
    }

    pub fn quests(&self) -> Option<Vec<(&Quest, &String, bool)>> {
        match &self.global.player.quests.assigned {
            Some(v) => {
                Some(v.iter()
                    .map(|s| {
                        let quest = self.quests.get(s).unwrap();
                        let completed = quest.check(&self.global.player);

                        (quest, s, completed)
                    })
                    .collect::<Vec<(&Quest, &String, bool)>>()
                )
            },
            None => None
        }
    }

    pub fn quest_book(&self) -> String {
        match self.quests() {
            Some(v) => {
                format!("\n{}",
                    v.iter()
                        .map(|d| format!("- {}{}",
                            d.0.name,
                            if d.2 { String::from(" ✔") } else { String::new() }
                        ))
                        .collect::<Vec<String>>()
                        .join("\n")
                )
            },
            None => String::from("You don't have any assigned quests.")
        }
    }
}