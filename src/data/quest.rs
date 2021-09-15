extern crate serde;

use serde::Deserialize;

use super::common::{Condition, Named};
use super::data::GameData;

#[derive(Debug, Deserialize, Clone)]
pub struct Quest {
    pub name: String,
    #[serde(alias = "goal")]
    pub description: String,
    pub requirements: Vec<Condition>
}

impl Named for Quest {
    fn name(&self) -> String {
        self.name.clone()
    }
}

impl Quest {
    pub fn check(&self, game: &GameData) -> bool {
        Condition::check_all(&self.requirements, game)
    }

    pub fn display(&self, game: &GameData) -> String {
        format!("{}\n\n{}\n\nRequirements:\n{}",
            self.name,
            self.description,
            Condition::display_all(&self.requirements, game).iter()
                .map(|c| format!("- {}", c))
                .collect::<Vec<String>>()
                .join("\n")
        )
    }
}