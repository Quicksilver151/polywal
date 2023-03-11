use crate::*;

pub mod betterdiscord;
pub mod godot;
pub mod polybar;
pub mod tabliss;

pub use betterdiscord::*;
pub use godot::*;
pub use polybar::*;
pub use tabliss::*;

pub enum App {BetterDiscord, Godot, Polybar, Tabliss}

pub fn write_palette_to(app: App, palette: &Palette) -> Result<(),FileError>{
    
    match app{
        App::BetterDiscord  => todo!(),
        App::Godot          => todo!(),
        App::Polybar        => todo!(),
        App::Tabliss        => tabliss::set_theme(),
    }
    
}
