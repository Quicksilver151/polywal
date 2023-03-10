#[derive(Debug, PartialEq)]
pub enum FlagState {
    Normal,
    Abnormal,
}


#[derive(Debug, PartialEq)]
pub struct Flag {
    pub help   : bool,
    pub edit   : bool,
    pub config : bool,

}

// default params
pub fn new_flag() -> Flag {
    Flag {
        help   : false,
        edit   : false,
        config : false,
    }
}

pub const HELP_TEXT : &str =
"Custom pywal implemetations for different software

Usage: polywal [OPTIONS]

Options:
    -h, --help                  shows this help section
    -e, --edit                  edits config
    -c, --config <filepath>     selects a config file


config is stored in ~/.config/polywal/";

pub fn parse_args(mut args: Vec<String>) -> Result<Flag, Flag> {
    // let mut args : Vec<String> = env::args().collect();
    
    args.reverse();
    args.pop();
    args.reverse();
    
    // println!("{:?}",args);
    
    let mut flag: Flag = new_flag();
    
    for arg in args {
        let arg_vec: Vec<char> = arg.chars().collect::<Vec<char>>();
        if arg_vec.len() == 1 {
            println!("===INVALID ARGS ENTERED===\n\n{}", HELP_TEXT);
            return Err(flag);
        }
        else if arg_vec[0] == '-' && arg_vec[1] == '-' {
            let argument = arg.strip_prefix("--").unwrap();
            match argument {
                "help"      => flag.help    = true,
                "edit"      => flag.edit    = true,
                "config"    => flag.config  = true,
                _ => {
                    println!("===INVALID FLAG ENTERED===\n\n{}", HELP_TEXT);
                    return Err(flag);
                }
            }
            
        }
        else if arg_vec[0] == '-' {
            for argchar in arg_vec {
                if argchar == '-' {
                    continue;
                }
                match argchar {
                    'h' => flag.help    = true,
                    'e' => flag.edit    = true,
                    't' => flag.config  = true,
                    _ => {
                        println!("==INVALID FLAG ENTERED===\n\n{}", HELP_TEXT);
                        return Err(flag);
                    }
                }
            }
        }
    }
    Ok(flag)
}


