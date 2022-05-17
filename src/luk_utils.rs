

use std::env;
use std::fs;
use std::io::stdin;

/**
 * ## Read Args
 * Function to get Aplication cmd Arguments */
pub fn  read_agrs()->Vec<String>{
    let args: Vec<String> = env::args().collect();
    return args;
}

/**
 * ## Read File
 * Function to read file by providing a path */
pub fn read_file(path:&str)-> String {
    let  res = fs::read_to_string(path).expect("Unable to read file");
    return String::from(res);
    
}

/**
 * ## Read Input
 * Extended stdin with witch the user can print a message before waiting for input */
pub fn read_input(message:Option<&str>)-> String{
    if Some(message) != Some(None) {
        print!("{0}: ", Some(message).unwrap().unwrap());
    }
    let mut input_string =  String::new();
    stdin().read_line(&mut input_string).ok().expect("Failed to read line");
    return String::from(input_string.replace('\n', "").trim());
}