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

fn find_word_count(file_path: &str, word: &str) -> i32 {
    let mut map = HashMap::new();
    let content = match get_file_content(file_path) {
        Ok(content) => content,
        Err(error) => {
            println!("Error reading file {}", error);
            return 0;
        }
    };

    let words = content.split_whitespace();
    for w in words {
        let count = map.entry(w).or_insert(0);
        *count += 1;
    }
    let count = map.entry(word).or_insert(0);
    *count
}

pub fn main(){
    let args = get_args();
    println!("args are {:?}", args);

    let file_path = &args[1];

    let word = &args[2];

    let count = find_word_count(file_path, word);

    println!("count of {} is {}", word, count);
}

