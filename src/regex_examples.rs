use regex::Regex;

fn main(){

    // match with pattern

    let rex = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
    let date = "2019-01-01";
    println!("Is {} a valid date? {}", date, rex.is_match(date));

    let email_rex = Regex::new(r"^[a-zA-Z0-9_]+@[a-zA-Z0-9_]+\.[a-zA-Z0-9_]+$").unwrap();

    let email = "amolkhatri@gmail.com";

    println!("Is {} a valid email? {}", email, email_rex.is_match(email));


    // match with pattern and capture

    let rex = Regex::new(r"^\d{4}-(\d{2})-(\d{2})$").unwrap();
    let date = "2019-01-01";


    if rex.is_match(date) {
        match rex.captures(date){
            Some(caps) => {
                println!("Month: {}", &caps[1]);
                println!("Day: {}", &caps[2]);
            },
            None => println!("No match")
        }
    }

    // replace with pattern

    let rex = Regex::new(r"^(\d{4})-(\d{2})-(\d{2})$").unwrap();
    let date = "2019-01-01";

    println!("Replaced date is {}", rex.replace(date, "$3/$2/$1"));

    //replace all matched patterns

    let rex = Regex::new(r"\d{4}").unwrap();
    let date = "2019-01-01";

    println!("Replaced date is {}", rex.replace_all(date, "XXXX"));
    




}