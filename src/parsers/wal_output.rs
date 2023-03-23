use crate::*;

pub fn file_content_from_wal(dir:&str) -> String {
    let mut file_path = home_dir().unwrap();
    file_path.push(".cache/wal/");
    file_path.push(dir);
    
    fs::read_to_string(file_path).expect("Should have been able to read the file")
}

pub fn file_content_from_home(dir:&str) -> String {
    let mut file_path = home_dir().unwrap();
    file_path.push(dir);
    
    fs::read_to_string(file_path).expect("Should have been able to read the file")
}


pub fn get_hex_colors() -> Palette {
    let mut file_path = home_dir().unwrap();
    file_path.push("colors");
    
    let contents = file_content_from_home(".cache/wal/colors");
    let contents: Vec<&str> = contents.split('\n').collect();
    
    Palette::from_vec(contents)
}

pub fn get_wallaper_path() -> String{
    file_content_from_wal("wal")
}

pub fn get_256_colors() {
    todo!()
}
