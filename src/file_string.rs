use std::panic;
use std::fs::File; 
use std::path::Path;
use std::io::prelude::*;

pub fn file_string(file: &str) -> Vec<String> { //return a vector of strings from an input assembly file 

let file_path = Path::new(file); //create new path to contain the path to the file
let display = file_path.display(); //set up file path display so that we can debug
let mut file = match File::open(&file_path){ // open the file
    Err(why) => panic!("Couldn't open {} : {}", display, why), //catch errors
    Ok(file) => file
};

let mut file_string = String::new(); //we're parsing the file to a string, create it

match file.read_to_string(&mut file_string) { // read the file to string
    Err(why) => panic!("Couldn't Read {} : {}", display, why), //catch errors 
    Ok(_) => (), 
}
let split_file_string: Vec<&str> = file_string.split("\n").collect();


let string_vec: Vec<String> = split_file_string
    .into_iter()
    .map(|s| s.to_string())
    .collect();

return string_vec;
}