// stds
pub use std::env;
pub use std::fs;


// crates 
pub use home::*;
pub use serde::{Serialize,Deserialize};
// files
mod structs;
mod parsers;
mod apps;

// use files
use structs::*;
use parsers::*;
use apps::*;

fn main() {
    // init
    // handle_ctrlc();
    
    // load config
    let cfg_result: Result<Config, confy::ConfyError> = confy::load("polywal", None);
    let cfg = match cfg_result {
        Ok(cfg_result) => cfg_result,
        Err(_cfg_result) => {
            println!("Warning: config was broken so it has been autofixed");
            Config::new()
        }
    };
       
    confy::store("polywal", None, cfg).unwrap();
    
    // fetch flags
    let args: Vec<String> = env::args().collect();
    let flag: Flag = match args::parse_args(args) {
        Ok(flag) => flag,
        Err(_flag) => return,
    };
    
    // main logic
    
    if flag.help {
        // breakout for help
        println!("{}", HELP_TEXT);
        return;
    }
    let wal_colors:Palette = get_hex_colors();
    println!("{}",wal_colors);
    
}
