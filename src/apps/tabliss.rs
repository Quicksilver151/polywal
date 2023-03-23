
use crate::*;

pub fn set_theme() -> Result<(), FileError>{
    let path = get_relative_path(
        ".mozilla/firefox/nvrfmscc.default-release/storage/default/moz-extension+++f05e6875-9650-477d-90ba-a753d8509a85^userContextId=4294967295/idb/3647222921wleabcEoxlt-eengsairo.files/4");
    //TODO: auto detect the directory or we're going nowhere
    
    let wallpaper = get_wallaper_path();
    
    fs::remove_file(&path).unwrap();
    std::os::unix::fs::symlink(wallpaper, path).unwrap();
    
    Ok(())
}
