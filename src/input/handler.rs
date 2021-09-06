use super::super::data::data::GameData;
use super::super::data::entity::PlayerStatus;
use super::super::data::location::LocationType;

use super::args::Args;
use super::controller::InputController;

pub fn handler(game: &mut GameData, input: &mut InputController) {
    use LocationType::*;
    use PlayerStatus::*;

    loop {
        match input.read() {
            Some(l) => {
                let line = l.trim();
                let args = Args::parse(&line);

                let e: Result<(), &str> = Err("You can't do that right now.");
                let cmd = args.command.as_str();

                let result = match &game.global.player.status {
                    Combat(c) => {
                        match cmd {
                            _ => e
                        }
                    }
                    House(h) => {
                        let house = game.house().unwrap();

                        match cmd {
                            "talk" => {
                                match input.choice("Who will you talk to?", house.list()) {
                                    Some(d) => {
                                        println!("\"{}\"", house.talk(d.1, game, input));
                                    }
                                    None => ()
                                }   

                                Ok(())
                            }
                            _ => e
                        }
                    }
                    Location(loc) => {
                        match cmd {
                            _ => e
                        }
                    }
                    Idle => {
                        match cmd {
                            _ => e
                        }
                    }
                };

                match result {
                    Ok(_) => (),
                    Err(e) => {
                        match cmd {
                            "quests" => {
                                println!("{}", game.quest_book())
                            }
                            _ => println!("{}", e)
                        }
                    }
                }

                println!();
            },
            None => break
        }
    }
}