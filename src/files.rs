use std::fs::File;
use std::io::Read;
fn main(){

    // read file content
    let mut file = match File::open("./src/temp.txt"){

        Ok(file) => file,

        Err(error) => {
            println!("error readign the file {}", error);
            return
        }
    };

    let mut content = String::new();

    match file.read_to_string(&mut content){
        Ok(_) => {
            println!("File content is {}", content);
        },
        Err(error) => {
            println!("Error reading file {}", error);
        }
    }
}