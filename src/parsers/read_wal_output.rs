
use crate::*;

pub fn get_hex_colors() -> Palette{
    dbg!(home_dir());
    let mut file_path = home_dir().unwrap();
    file_path.push(".cache/wal/colors");
    
    // let file_path: PathBuf = PathBuf::from("~/.cache/wal/colors");
    println!("In file {:?}", file_path.file_name().unwrap());
    
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    
    let contents : Vec<&str> = contents.split('\n').collect();
    
    Palette::new_from_vec(contents)
    
}

pub fn get_256_colors(){
    todo!()
}
pub fn get_wallpaper_path() -> String{
    todo!();
}
