mod luk_utils;
use luk_utils::read_input;
use luk_utils::read_file;

mod luk_mysql;
use luk_mysql::mysql_connect;


fn main() {
    
    welcome();
    mysql_connect(|res:Vec<luk_mysql::Dados>|{
        for table_name in res {
            println!("{}",table_name.table_name);
        }
    });

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









      
