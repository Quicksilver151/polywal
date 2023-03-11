use crate::*;

pub fn set_theme() -> Result<(), FileError>{
    let path = get_relative_path(
        ".mozilla/firefox/nvrfmscc.default-release/storage/default/moz-extension+++f05e6875-9650-477d-90ba-a753d8509a85^userContextId=4294967295/idb/3647222921wleabcEoxlt-eengsairo.files/4");
    
    println!("tabliss path:\n{}",path.to_str().unwrap());
    Ok(())
}
