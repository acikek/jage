mod data;
mod fs;
mod input;

use input::controller::*;
use input::handler::handler;
use fs::fs::Filesystem;
use data::data::GameData;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = InputController::new(String::from("> "));

    let fs = Filesystem::new(String::from("/home/acikek/Desktop/Projects/rust/jage/game"));
    let mut game = GameData::from(&fs)?;

    handler(&mut game, &mut input);

    Ok(())
}
