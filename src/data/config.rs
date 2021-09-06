extern crate serde;
extern crate serde_yaml;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Metadata {
    pub title: String,
    pub description: String,
    pub authors: Vec<String>
}

#[derive(Debug, Deserialize)]
pub struct CurrencyData {
    pub singular: String,
    pub plural: String,
    pub symbol: String,
    pub dist_cost: usize
}

#[derive(Debug, Deserialize)]
pub struct World {
    pub name: String,
    pub description: String,
    pub currency: CurrencyData
}

#[derive(Debug, Deserialize)]
pub struct Game {
    pub prompt: String,
    pub exposition: String,
    pub world: World
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub metadata: Metadata,
    pub game: Game
}