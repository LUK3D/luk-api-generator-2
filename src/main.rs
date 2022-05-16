use std::env;
use std::fs;
use std::io::stdin;

use mysql::*;
use mysql::prelude::*;



fn main() {
    
    welcome();


    mysql_connect();



    let  db:String;
    let  path:String;
    println!("|______________________________________________________________|");
    db = read_input(Some("|Informe o nome da Base de Dados"));
    println!("|______________________________________________________________|");
    path = read_input(Some("|Informe a localização do Projecto"));
    println!("|______________________________________________________________|");
    println!("\n|Criando a API {0}Em {1} ",db, path );
    println!("|______________________________________________________________|");
}

/**Function to print a welcome message */
fn welcome(){
    println!("{}",read_file("./welcome.txt"));
}


pub fn  read_agrs()->Vec<String>{
    let args: Vec<String> = env::args().collect();
    return args;
}

pub fn read_file(path:&str)-> String {
    let  res = fs::read_to_string(path).expect("Unable to read file");
    return String::from(res);
    
}

pub fn read_input(message:Option<&str>)-> String{

    if Some(message) != Some(None) {
        println!("{0}: ", Some(message).unwrap().unwrap());
    }
    let mut input_string =  String::new();
    stdin().read_line(&mut input_string).ok().expect("Failed to read line");
    return input_string;
}

pub fn mysql_connect(){
    let opts = mysql::OptsBuilder::new()
    .user(Some("root"))
    .pass(Some(""))
    .ip_or_hostname(Some("127.0.0.1"))
    .db_name(Some("teste1"));
    
    let pool = mysql::Pool::new(opts).unwrap();

    let mut conn = pool.get_conn().unwrap();
        
    let selected_payments = conn.query_iter("SELECT id, name from comments").unwrap().for_each(|row| {
        let r:(i32, String) = from_row(row.unwrap());
        println!("{}, {}", r.0, r.1);
      });
}