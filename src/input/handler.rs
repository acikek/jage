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
                        game.cycle_combat();

                        match cmd {
                            _ => e
                        }
                    }
                    House(_) => {
                        let house = game.house().unwrap().clone();

                        match cmd {
                            "talk" => {
                                match input.choice("Who will you talk to?", house.list(), "You decided not to talk to anyone.") {
                                    Some(d) => {
                                        match house.talk(d.1, game, input) {
                                            Some(s) => println!("{}", s),
                                            None => ()
                                        }
                                    }
                                    None => ()
                                }   

                                Ok(())
                            }
                            "leave" => {
                                game.global.player.status = PlayerStatus::Location;

                                println!("You left the house.");

                                Ok(())
                            }
                            _ => e
                        }
                    }
                    Location => {
                        match cmd {
                            "travel" => {
                                if args.check(1) {
                                    let matched = game.match_best(&args.input, &game.locations);

                                    match matched {
                                        Some(d) => game.travel(&d.0, &d.1, input),
                                        None => println!("That's not a valid location.")
                                    }
                                } else {
                                    println!("You need to provide a location.");
                                }

                                Ok(())
                            }
                            "visit" => {
                                use LocationType::*;

                                match game.location().l_type {
                                    Town(_) | City(_) | Capital(_) => {
                                        game.visit(input);

                                        Ok(())
                                    }
                                    _ => e
                                }
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
                            "dbg" => println!("\n{:#?}", game),
                            "inv" => println!("\n{}", game.global.player.inventory.display_line(game)),
                            "log" => println!("\n{}", game.global.player.stats.log(false)),
                            "recent" => println!("\n{}", game.global.player.stats.log(true)),
                            "quest" => {
                                if args.check(1) {
                                    match game.match_best(&args.input, &game.quests) {
                                        Some(q) => println!("\n{}", q.1.display(game)),
                                        None => println!("That's not a valid quest.")
                                    }
                                } else {
                                    println!("You need to provide a quest.");
                                }
                            }
                            "quests" => println!("\n{}", game.quest_book()),
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