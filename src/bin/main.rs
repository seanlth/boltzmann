
extern crate boltzmann;
extern crate rand;
extern crate clap;
extern crate commands;
extern crate rustyline;


use rustyline::Editor;

use clap::{Arg, App, SubCommand};

use commands::parser::{Command, CommandTree, ParseError, Parser};
use commands::tokenizer::tokenize;

use boltzmann::simulator::Simulator;
use boltzmann::collision::SpatialPartition;
use boltzmann::vector::*;
use boltzmann::attribute::*;
use boltzmann::drawing::*;

use boltzmann::spatial_hash::SpatialHash;
use boltzmann::config::*;


// boltzmann new 
// boltzmann open
//
// run
// stop
// snapshot [-n t]
// particles
// plotter
// density

fn main() {
    let mut app = App::new("boltzmann\n")
                          .version("0.1")
                          .about("Granular dynamics simulator")
                          .subcommand(SubCommand::with_name("new")
                                      .about("Create a new simulation")
                                      .arg_from_usage("<NAME>   'Set simulation name'"))
                          .subcommand(SubCommand::with_name("open")
                                      .about("Open simulation."));

    
    let matches = app.get_matches();
    println!("{:?}", matches);
    println!("{:?}", matches.subcommand_matches("new").unwrap().value_of("NAME"));


    //app.print_help();
    

    let mut tree = CommandTree::new();
    tree.command(Command::new("run"));
    tree.command(Command::new("stop"));
    
    let root = tree.finalize();

    let mut rl = rustyline::Editor::<()>::new();
    while let Ok(line) = rl.readline("") {
        rl.add_history_entry(&line);
        if let Ok(tokens) = tokenize("run") {
            println!("{:?}", tokens);
            let mut parser = Parser::new(root.clone());
            if let Err(err) = parser.parse(tokens) {
                match err {
                    ParseError::NoMatches(_, acceptable) => {
                        println!("No match for '{}'", line);
                        println!("\nPossible options:");
                        for ref option in acceptable {
                            let n = option.node();
                            println!("  {} - {}", n.help_symbol, n.help_text);
                        }
                    }
                    ParseError::AmbiguousMatch(_, matches) => {
                        println!("\nCan be interpreted as:");
                        for ref option in matches {
                            let n = option.node();
                            println!("  {} - {}", n.help_symbol, n.help_text);
                        }
                    }
                }
            } 
            else if let Err(err) = parser.verify() {
                println!("err:");
                println!("{}", err);
            } 
            else {
                //parser.execute();
            }
        }
        println!("");
    }
    println!("\nExiting.");


}
