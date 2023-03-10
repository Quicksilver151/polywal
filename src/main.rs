// stds
pub use std::env;
pub use std::fs;


// crates 
pub use home::*;

// files
mod parsers;
use crate::parsers::*;

fn main() {
    let new_palette:Palette = get_hex_colors();
    dbg!(new_palette.color1);
    
    
    println!("Hello, world! in {}",new_palette.color0.to_string());
}
