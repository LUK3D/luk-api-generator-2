
// use mysql::*;
use mysql::prelude::*;

#[path = "types.rs"]
pub mod types;
pub use types::Dados;
pub use types::DbConfig;



pub fn create_pool(config:DbConfig) -> mysql::Pool{

    let mut conf = config;

    if config.host.len()==0 {
            config.host =  "102.0.0.1".to_string();
    }
    
    let opts = mysql::OptsBuilder::new()
    .user(Some(conf.user))
    .pass(Some(conf.pwd).unwrap())
    .ip_or_hostname(Some(conf.host).unwrap())
    .db_name(Some(conf.db));
    
    return mysql::Pool::new(opts).unwrap();
}




/**Function to connect to mysql server and get some data */
pub fn mysql_connect(pool:mysql::Pool, callback: fn(Vec<Dados>)){
   
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