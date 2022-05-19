

use std::path::PathBuf;
use std::env;
use std::fs;
use std::io::stdin;
use convert_case::{Case, Casing};
use std::ops::{Bound, RangeBounds};

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
        println!("{0}: ", Some(message).unwrap().unwrap());
    }
    let mut input_string =  String::new();
    stdin().read_line(&mut input_string).ok().expect("Failed to read line");
    return String::from(input_string.replace('\n', "").trim());
}


pub fn get_current_working_dir() -> std::io::Result<PathBuf> {
    env::current_dir()
}




trait StringUtils {
    fn substring(&self, start: usize, len: usize) -> &str;
    fn slice(&self, range: impl RangeBounds<usize>) -> &str;
}

impl StringUtils for str {
    fn substring(&self, start: usize, len: usize) -> &str {
        let mut char_pos = 0;
        let mut byte_start = 0;
        let mut it = self.chars();
        loop {
            if char_pos == start { break; }
            if let Some(c) = it.next() {
                char_pos += 1;
                byte_start += c.len_utf8();
            }
            else { break; }
        }
        char_pos = 0;
        let mut byte_end = byte_start;
        loop {
            if char_pos == len { break; }
            if let Some(c) = it.next() {
                char_pos += 1;
                byte_end += c.len_utf8();
            }
            else { break; }
        }
        &self[byte_start..byte_end]
    }
    fn slice(&self, range: impl RangeBounds<usize>) -> &str {
        let start = match range.start_bound() {
            Bound::Included(bound) | Bound::Excluded(bound) => *bound,
            Bound::Unbounded => 0,
        };
        let len = match range.end_bound() {
            Bound::Included(bound) => *bound + 1,
            Bound::Excluded(bound) => *bound,
            Bound::Unbounded => self.len(),
        } - start;
        self.substring(start, len)
    }
}






pub fn snakeToCamel(val:&str)->String{

    let parts = val.split("_");
    let mut camelWord:String = "".to_owned();
    for word in parts {
        let l = word.substring(0,1);
        let mut w = "";
        if word.len()>1{
            w = word.substring(1,word.len());
        }
        camelWord.push_str(&l.to_string().to_uppercase());
        camelWord.push_str( &w.to_string().to_lowercase());
    }
    return camelWord.to_string();
 
 }






 