use std::io;
use std::io::prelude::*;

mod luk_utils;
use luk_utils::read_input;
use luk_utils::read_file;
mod luk_mysql;

use luk_mysql::mysql_connect;
use luk_mysql::create_pool;
use luk_mysql::types::DbConfig;



fn main() {

    welcome();

    let  db:String;
    let  path:String;
    println!("|______________________________________________________________|");
    db = read_input(Some("|Informe o nome da Base de Dados"));
    println!("|______________________________________________________________|");
    path = read_input(Some("|Informe a localização do Projecto"));
    println!("|______________________________________________________________|");
    println!("\n|Criando a API {0}Em {1} ",db, path );
    println!("|______________________________________________________________|");
    println!("| {}",db);
    let mut tables: Vec<String> = vec![];
    tables = mysql_connect(create_pool(db_config(&db)),processar);

    for t in tables {
        println!("|______________________________________________________________|");
        println!("| TABELA: {}",t.to_string());
    }
}

fn processar(res:Vec<luk_mysql::Dados>)->Vec<String>{
    let mut tables: Vec<String> = vec![];
    for elem in res {
        if !tables.contains(&elem.table_name.to_string()){
            tables.push(elem.table_name.to_string());
        }
    }
    return tables;
}


/**Creates new database configuration instance. */
fn db_config(db:&str) -> DbConfig{
    return DbConfig{
        db: String::from(db),
        user: String::from("root"),
        pwd:String::from(""),
        host:String::from("127.0.0.1")
    };
}



/**Function to print a welcome message */
fn welcome(){
    println!("{}",read_file("./welcome.txt"));
}



fn pause() {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    // We want the cursor to stay at the end of the line, so we print without a newline and flush manually.
    write!(stdout, "Press any key to continue...").unwrap();
    stdout.flush().unwrap();

    // Read a single byte and discard
    let _ = stdin.read(&mut [0u8]).unwrap();
}




      
