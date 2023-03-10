// stds
pub use std::env;
pub use std::fs;


// crates 
pub use home::*;

// files
mod structs;
mod parsers;
mod apps;

// use files
use structs::*;
use parsers::*;
use apps::*;

fn main() {
    let wal_colors:Palette = get_hex_colors();
    dbg!(wal_colors.to_vec());
    
}
