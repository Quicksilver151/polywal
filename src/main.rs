// stds
pub use std::env;
pub use std::fs;

// crates
pub use home::*;
pub use owo_colors::*;
pub use serde::{Deserialize, Serialize};

// files
mod apps;
mod parsers;
mod structs;

// use files
use apps::*;
use parsers::*;
use structs::*;

fn main() {
    // init
    // handle_ctrlc();

    // load config
    let cfg_result: Result<Config, confy::ConfyError> = confy::load("polywal", None);
    let cfg = match cfg_result {
        Ok(cfg_result) => cfg_result,
        Err(cfg_result) => {
            println!("Warning: config was broken so it has been autofixed");
            dbg!(cfg_result);
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

    // help break
    if flag.help {
        // breakout for help
        println!("{}", HELP_TEXT);
        return;
    }

    // main logic
    // ==========

    let wal_colors: Palette = get_hex_colors();
    write_palette_to(App::Tabliss, &wal_colors);
    println!("{}", &wal_colors);
}
