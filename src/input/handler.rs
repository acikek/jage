use super::super::data::data::GameData;
use super::super::data::entity::PlayerStatus;
use super::super::data::location::LocationType;

use super::args::Args;
use super::controller::InputController;

pub fn handler(game: &mut GameData, input: &mut InputController) {
    use LocationType::*;
    use PlayerStatus::*;
    
    let mut exiting = false;

    loop {
        match input.read() {
            Some(l) => {
                if exiting {
                    exiting = false;
                }

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
                                match input.choice("Who will you talk to?", house.list(), "You decided not to talk to anyone.") {
                                    Some(d) => {
                                        match house.talk(d.1, game, input) {
                                            Some(s) => println!("\"{}\"", s),
                                            None => ()
                                        }
                                    }
                                    None => ()
                                }   

                                Ok(())
                            }
                            _ => e
                        }
                    }
                    Location => {
                        match cmd {
                            "travel" => {
                                if args.check(1) {
                                    match game.match_location(&args.input) {
                                        Some(d) => {
                                            game.travel(d.1, d.0, input)
                                        }
                                        None => println!("That's not a valid location.")
                                    }
                                } else {
                                    println!("You need to provide a location.");
                                }

                                Ok(())
                            }
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
            None => {
                if !exiting {
                    println!("Press Ctrl+C again to exit.\nAll your progress will be saved.\n");
                    exiting = true;
                } else {
                    break;
                }
            }
        }
    }
}