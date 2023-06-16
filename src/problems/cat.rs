use std::env;
use std::fs::File;
use std::io::Read;
use std::io::Result;
use std::collections::HashMap;

fn get_args() -> Vec<String> {
    let args: Vec<String> = env::args().collect();
    args
}

fn get_file_content(file_path: &str) -> Result<String>{

    let mut file = match File::open(file_path){
        Ok(file) => file,
        Err(error) => {
            println!("error readign the file {}", error);
            return Err(error);
        }
    };

    let mut content = String::new();
    match file.read_to_string(&mut content){
        Ok(_) => {
            println!("File content is {}", content);
        },
        Err(error) => {
            println!("Error reading file {}", error);
            return Err(error);
        }
    }
    Ok(content)
}

pub fn main(){
    let args = get_args();
    println!("args are {:?}", args);

    let file_path = &args[1];

    let content = match get_file_content(file_path) {
        Ok(content) => content,
        Err(error) => {
            println!("Error reading file {}", error);
            return;
        }
    };

    println!("\n {:?}", content);
}

