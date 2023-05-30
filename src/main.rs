mod storage;

use std::io;
use std::io::Read;
use std::fs;

pub use crate::storage::Storage;


fn get_user_input() -> Result<String, io::Error> {
    let filename = "input.txt";

    let mut file = fs::File::open(&filename)?;

    let mut data = String::new();
    file.read_to_string(&mut data)?;
    Ok(data)
}




fn main() {
    let mut st = storage::Storage::new();
    
    let uinput = get_user_input().expect("something went wrong while reading user input!");
    
    let commands = uinput.split("\n");
    
    for cmd in commands.into_iter() {
        println!("action: {}", cmd);
        let cmd : Vec<&str> =  cmd.split(" ").collect();
        let action = cmd[0];
        let key = cmd[1];
        let mut value = "";

        if cmd.len() > 2 {
            value = cmd[2]
        }
        
        match action {
            "set" => {
                st.set(String::from(key), String::from(value), false).expect("something went wrong while setting value!");
                println!("set successfully!")
            },
            "get" => {
                let value = st.get(String::from(key)).expect("something went wrong while getting value!");
                println!("got: {}", value);
            },
            &_ => todo!(),
        }
        
    }
    
}
