use std::fmt::Display;

use crate::*;

#[derive(Default, Debug, Serialize, Deserialize)]
pub enum Godot {#[default] False, Light, Color, Dark}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Config{
    pub discord : bool,
    pub tabliss : bool,
    pub polybar : bool,
    pub godot   : Godot,
}
impl Config{
    pub fn new() -> Config{
        Config { discord: false, tabliss: false, polybar: false, godot: Godot::False }
    }
}



#[derive(Debug)]
pub struct Color(pub String);

impl Color{
    pub fn new() -> Color{
        Color("#000000".to_string())
    }
}

impl Display for Color{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) ->std::result::Result<(), std::fmt::Error> {
        let Color(output) = &self;
        write!(f, "{output}")?;
        Ok(())
    }
}

// trait GetColor{
//     fn get_color(self) -> Color;
// }
// impl GetColor for &str{
//     fn get_color(self) -> Color{
//         Color(self.to_string())
//     }
// }

#[derive(Debug)]
pub struct Palette{
    pub color0  : Color,
    pub color1  : Color,
    pub color2  : Color,
    pub color3  : Color,
    pub color4  : Color,
    pub color5  : Color,
    pub color6  : Color,
    pub color7  : Color,
    pub color8  : Color,
    pub color9  : Color,
    pub color10 : Color,
    pub color11 : Color,
    pub color12 : Color,
    pub color13 : Color,
    pub color14 : Color,
    pub color15 : Color,
}

impl Palette{
    pub fn new() -> Palette{
            Palette{
            color0  : Color::new(),
            color1  : Color::new(),
            color2  : Color::new(),
            color3  : Color::new(),
            color4  : Color::new(),
            color5  : Color::new(),
            color6  : Color::new(),
            color7  : Color::new(),
            color8  : Color::new(),
            color9  : Color::new(),
            color10 : Color::new(),
            color11 : Color::new(),
            color12 : Color::new(),
            color13 : Color::new(),
            color14 : Color::new(),
            color15 : Color::new(),
        }
    }
    
    pub fn from_vec(vec: Vec<&str>) -> Palette{
        Palette{
            
            color0  : Color(vec[0 ].to_string()),
            color1  : Color(vec[1 ].to_string()),
            color2  : Color(vec[2 ].to_string()),
            color3  : Color(vec[3 ].to_string()),
            color4  : Color(vec[4 ].to_string()),
            color5  : Color(vec[5 ].to_string()),
            color6  : Color(vec[6 ].to_string()),
            color7  : Color(vec[7 ].to_string()),
            color8  : Color(vec[8 ].to_string()),
            color9  : Color(vec[9 ].to_string()),
            color10 : Color(vec[10].to_string()),
            color11 : Color(vec[11].to_string()),
            color12 : Color(vec[12].to_string()),
            color13 : Color(vec[13].to_string()),
            color14 : Color(vec[14].to_string()),
            color15 : Color(vec[15].to_string()),
        }
    }
    pub fn to_vec(&self) -> Vec<String>{
        vec![
            self.color0 .to_string(),
            self.color1 .to_string(),
            self.color2 .to_string(),
            self.color3 .to_string(),
            self.color4 .to_string(),
            self.color5 .to_string(),
            self.color6 .to_string(),
            self.color7 .to_string(),
            self.color8 .to_string(),
            self.color9 .to_string(),
            self.color10.to_string(),
            self.color11.to_string(),
            self.color12.to_string(),
            self.color13.to_string(),
            self.color14.to_string(),
            self.color15.to_string(),
        ]
    }
}

impl Display for Palette{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) ->std::result::Result<(), std::fmt::Error> {
        let mut output = self.to_vec();
        write!(f,"{}",output[0])?;
        output.reverse();
        output.pop();
        output.reverse();
        output.iter().try_for_each(|x| write!(f,"\n{x}"))?;
        Ok(())
    }
}

pub enum FileError {FileNotFound, WritingFailed, ReadingFailed}
