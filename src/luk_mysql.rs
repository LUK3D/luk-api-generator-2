
// use mysql::*;
use mysql::prelude::*;

#[path = "types.rs"]
pub mod types;
pub use types::Dados;




/**Function to connect to mysql server and get some data */
pub fn mysql_connect(callback: fn(Vec<Dados>)){
    let opts = mysql::OptsBuilder::new()
    .user(Some("root"))
    .pass(Some(""))
    .ip_or_hostname(Some("127.0.0.1"))
    .db_name(Some("teste1"));
    
    let pool = mysql::Pool::new(opts).unwrap();
    let mut conn = pool.get_conn().unwrap();
        
    /* 
        let selected_payments = conn.query_iter("
        select   TABLE_NAME  from information_schema.columns
        where table_schema = 'teste1'
        order by table_name,ordinal_position
    
        ").unwrap().for_each(|row| {
            // let r:(i32, String) = from_row(row.unwrap());
            println!("{:?}", row.unwrap());

        });
    */

      let res = conn.query_map(
        "
        select   table_name  from information_schema.columns
        where table_schema = 'teste1'
        order by table_name,ordinal_position
        ",
        |table_name| Dados {table_name:table_name}).expect("Query failed.");
     
 
    callback(res);
}