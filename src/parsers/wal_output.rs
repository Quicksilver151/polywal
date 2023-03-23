use crate::*;

pub fn get_hex_colors() -> Palette {
    let mut file_path = home_dir().unwrap();
    file_path.push(".cache/wal/colors");
    
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    
    let contents: Vec<&str> = contents.split('\n').collect();
    
    Palette::from_vec(contents)
}

pub fn get_wallaper_location() -> String{
    todo!();
}

pub fn get_256_colors() {
    todo!()
}
pub fn get_wallpaper_path() -> String {
    todo!();
}
