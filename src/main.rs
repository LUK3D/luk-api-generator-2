use std::io;
use std::io::prelude::*;
use std::fs;

mod luk_utils;
use luk_utils::read_input;
use luk_utils::read_file;

mod luk_mysql;
use luk_mysql::mysql_connect;
use luk_mysql::create_pool;
use luk_mysql::types::DbConfig;
use luk_mysql::types::TableName;
use luk_mysql::types::TableInfo;

mod luk_generators;
use luk_generators::gen_structure;
use luk_generators::gen_routers;
use luk_generators::gen_controllers;
use luk_generators::gen_models;
use luk_generators::gen_resources;



fn main() {

    welcome();

    let mut project_path = "N:\\rust\\luk-api-generator\\src\\dist";
  

    let  db:String;
    let  path:String;
    println!("______________________________________________________________");
    db = read_input(Some("Enter the name of the Database"));
    println!("______________________________________________________________");
    path = read_input(Some("Enter the output Absolute Path"));
    println!("______________________________________________________________");

    if path.trim().len()>1{
        project_path = &path;
    }

    println!("\nCriando a API {0} Em {1} ",db, project_path );
    println!("______________________________________________________________");

      /**Generating project structure */
      gen_structure(project_path,||{});

    let mut tables: Vec<String> = vec![];
    tables = mysql_connect(create_pool(db_config(&db)),
    "
    select  TABLE_NAME  from information_schema.columns
    where table_schema = 'teste1'
    order by table_name,ordinal_position
    "
    ,processar);

    let mut table_info = TableInfo{
        names: vec![]
    };


    for t in tables {
        // println!("|______________________________________________________________|");
        // println!("| TABELA: {}",t.to_string());

        let t_name = TableName{
            lower:t.to_string(),
            upper:luk_utils::snakeToCamel(&t.to_string()).to_string(),
            columns: mysql_connect(create_pool(db_config(&db)),
            &format!("
            select  COLUMN_NAME  from information_schema.columns
            where table_schema = '{0}' and TABLE_NAME = '{1}'
            order by table_name,ordinal_position
            ",&db,&t.to_string())
            ,processar)
        };

        table_info.names.push(t_name);
        // fs::write(&format!("{0}\\controllers\\{1}Controller.php",project_path,table_info.names[0].upper), &gen_controllers(t_name)).unwrap();
    }

    pause();

    /** Gerando os arquivos ------------------------------------ --------------------------------------*/
    fs::write(&format!("{0}\\route\\api.php",project_path), &gen_routers(&table_info.names)).unwrap();
    for table in table_info.names {
        fs::write(&format!("{0}\\controllers\\{1}Controller.php",project_path,table.upper), &gen_controllers(&table)).unwrap();
        fs::write(&format!("{0}\\models\\{1}.php",project_path,table.upper), &gen_models(&table)).unwrap();
        fs::write(&format!("{0}\\resources\\{1}Resource.php",project_path,table.upper), &gen_resources(&table)).unwrap();
    }
    println!("______________________________________________________________");
    println!("DONE!");
    
    pause();


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




      
