extern crate serde;

use serde::{Deserialize, Serialize};

use std::collections::HashMap;

use super::attribute::{Class, Skill};
use super::config::Config;
use super::common::{InteractionLine, Named};
use super::entity::{Player, PlayerStatus};
use super::inventory::{Item, Currency};
use super::location::{Location, House};
use super::quest::Quest;
use super::time::GameTime;
use super::super::input::controller::InputController;
use super::super::fs::fs::Filesystem;

#[derive(Debug, Deserialize, Serialize)]
pub struct GlobalData {
    pub player: Player,
    pub time: GameTime
}

#[derive(Debug)]
pub struct GameData {
    pub config: Config,
    pub classes: HashMap<String, Class>,
    pub houses: HashMap<String, House>,
    pub items: HashMap<String, Item>,
    pub locations: HashMap<String, Location>,
    pub skills: HashMap<String, Skill>,
    pub quests: HashMap<String, Quest>,
    pub global: GlobalData
}

impl GameData {
    pub fn from(fs: &Filesystem) -> Result<Self, Box<dyn std::error::Error>> {
        let config = Filesystem::parse(fs.read("jage.yml")?)?;
        let classes = Filesystem::parse_map(fs.read_dir("classes")?)?;
        let houses = Filesystem::parse_map(fs.read_dir("houses")?)?;
        let items = Filesystem::parse_map(fs.read_dir("items")?)?;
        let locations = Filesystem::parse_map(fs.read_dir("locations")?)?;
        let skills = Filesystem::parse_map(fs.read_dir("skills")?)?;
        let quests = Filesystem::parse_map(fs.read_dir("quests")?)?;
        let global = Filesystem::parse(fs.read("data/global.yml")?)?;

        Ok(Self {
            config,
            classes,
            houses,
            items,
            locations,
            skills,
            quests,
            global
        })
    }

    pub fn location(&self) -> &Location {
        self.locations.get(&self.global.player.location).unwrap()
    }

    pub fn match_best<T: Named + Clone>(&self, matcher: &String, m: &HashMap<String, T>) -> Option<(String, T)> {
        for (k, v) in m {
            if k == &matcher.to_lowercase() || v.name().to_lowercase() == matcher.to_lowercase() {
                return Some((k.clone(), (*v).clone()))
            }
        }
        
        None
    }

    pub fn travel(&mut self, id: &String, next: &Location, input: &mut InputController) {
        let current = self.location();
        let data = current.travel_prompt(&next, self);

        let choices = vec!["Ride carriage", "Walk"].iter()
            .map(|s| String::from(*s))
            .collect::<Vec<String>>();

        let result = input.choice(data.0.as_str(), choices, "You decided not to travel.");

        match result {
            Some(d) => {
                match d.1 {
                    0 => {
                        match self.global.player.inventory.currency.add(-data.1, Some(&self.config.world.currency.plural)) {
                            Ok(_) => println!("You paid the fee. You now have {}.", Currency::display(self.global.player.inventory.currency.value, self)),
                            Err(e) => println!("{}", e)
                        }
                    }
                    1 => {
                        self.global.time.advance(data.2);
                        println!("You decided to walk.");
                    }
                    _ => ()
                }

                let entry = next.entry(id, &self.global.player.stats.reputation);
                
                if entry.2 {
                    self.global.player.stats.reputation.insert(id.clone(), entry.1);
                }

                println!("\n{}\n{}", entry.0, self.global.time);

                self.global.player.location = id.clone();
            },
            None => ()
        }
    }

    pub fn house(&self) -> Option<&House> {
        if let PlayerStatus::House(h) = &self.global.player.status {
            Some(self.houses.get(h).unwrap())
        } else {
            None
        }
    }

    pub fn visit(&mut self, input: &mut InputController) {
        let houses = self.location().houses(self).unwrap();

        let names = houses.iter()
            .map(|d| d.1.clone())
            .collect::<Vec<String>>();

        match input.choice("Which house will you visit?", names, "You decided not to visit anyone.") {
            Some(d) => {
                let house = &houses[d.1];
                self.global.player.status = PlayerStatus::House(house.0.clone());

                println!("You entered the {}.\n\n{}", 
                    house.1,
                    InteractionLine::all(&self.house().unwrap().entry)
                );
            }
            None => ()
        }
    }

    pub fn finished_quests(&self) -> Option<Vec<String>> {
        match &self.global.player.quests.assigned.clone() {
            Some(v) => {
                Some(
                    v.iter()
                        .map(|q| q.clone())
                        .filter(|q| self.quests.get(q).unwrap().check(self))
                        .collect::<Vec<String>>()
                )
            },
            None => None
        }
    }

    /*pub fn sort_quests(&mut self) {
        let completed: Vec<bool> = match &self.global.player.quests.assigned {
            Some(v) => {
                v.iter()
                    .map(|q| {
                        let quest = self.quests.get(&q.clone()).unwrap().clone();
                        quest.check(self)
                    })
                    .collect::<Vec<bool>>()
            }
            None => return
        };

        if completed.len() > 0 {
            if let Some(v) = &mut self.global.player.quests.assigned {
                for (i, q) in v.clone().iter().enumerate() {
                    if completed[i] {
                        v.retain(|s| s != &q.clone());

                        println!("You have completed the quest \"{}\"!", self.quests.get(&q.clone()).unwrap().clone().name);
                        
                        if let Some(c) = &mut self.global.player.quests.completed {
                            c.push(q.clone());
                        } else {
                            self.global.player.quests.completed = Some(vec![q.clone()]);
                        }
                    }
                }
            }
        }
    }*/

    /*pub fn quest_list(&mut self, v: Option<Vec(Quest, String)>>) -> String {

    }*/

    pub fn quest_list(&self, v: &Option<Vec<String>>) -> Option<Vec<(Quest, String)>> {
        match &v {
            Some(v) => {
                Some(
                    v.iter()
                        .map(|q| {
                            let quest = self.quests.get(q).unwrap();

                            (quest.clone(), q.clone())
                        })
                        .collect::<Vec<(Quest, String)>>()
                )
            }
            None => None
        }
    }

    pub fn display_quests(&self, v: &Option<Vec<(Quest, String)>>, show_completed: bool) -> String {
        match v {
            Some(q) => {
                format!("\n{}",
                    q.iter()
                        .map(|d| format!("- {}{}",
                            d.0.name,
                            if show_completed {
                                if d.0.check(self) {
                                    String::from(" ✔")
                                } else {
                                    String::new()
                                }
                            } else {
                                String::new()
                            }
                        ))
                        .collect::<Vec<String>>()
                        .join("\n")
                )
            }
            None => String::from("\nNothing here...")
        }
    }

    pub fn quest_book(&mut self) -> String {
        format!("Assigned:\n{}\n\nCompleted:\n{}",
            self.display_quests(&self.quest_list(&self.global.player.quests.assigned), true),
            self.display_quests(&self.quest_list(&self.global.player.quests.completed), false)
        )
    }
}