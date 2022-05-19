use crate::TableName;
use std::fs;
#[path="luk_utils.rs"]
mod luk_utils;
use luk_utils::read_file;
use std::env;

/**Generate Routes */
pub fn gen_routers(tables:&Vec<TableName>)->String{

    let mut routes:Vec<String> = vec![];
    let mut references:Vec<String> = vec![];
    let dir = env::current_dir().unwrap();

    let mut content = read_file(&format!("{0}\\src\\templates\\route.luk", dir.display()));
    for table in tables {
        routes.push(format!("Route::resource('/{0}', {1}Controller::class);", table.lower, table.upper).to_string());
        references.push(format!("use App\\Http\\Controllers\\{0}Controller;", table.upper).to_string());
    }

    content = content.replace("{FIELDS_VALUES}", &routes.join("\n"));
    content = content.replace("{IMPORTS}", &references.join("\n"));
    return content;
}

/**Generate Routes */
pub fn gen_controllers(table:&TableName)->String{

    let mut columns:Vec<String> = vec![];
    let dir = env::current_dir().unwrap();
    

   

    let mut content = read_file(&format!("{0}\\src\\templates\\controller.luk", dir.display()));
    for column in &table.columns {
        columns.push(format!("${0}->{1} = request->{1};", table.upper, column).to_string());
    }

    content = content.replace("{FIELDS_VALUES}", &columns.join("\n"));
    content = content.replace("{TABLE_U}", &table.upper);
    content = content.replace("{TABLE_L}", &table.lower);
    return content;
}

/**Generate Models */
pub fn gen_models(table:&TableName)->String{

    let mut columns:Vec<String> = vec![];
    let dir = env::current_dir().unwrap();

    let mut content = read_file(&format!("{0}\\src\\templates\\model.luk", dir.display()));
    for column in &table.columns {
        columns.push(format!("'{0}',", column).to_string());
    }

    content = content.replace("{FIELDS_VALUES}", &columns.join("\n"));
    content = content.replace("{TABLE_U}", &table.upper);
    content = content.replace("{TABLE_L}", &table.lower);
    return content;
}

/**Generate resources */
pub fn gen_resources(table:&TableName)->String{
    let dir = env::current_dir().unwrap();
    let mut content = read_file(&format!("{0}\\src\\templates\\resource.luk", dir.display()));
    content = content.replace("{TABLE_U}", &table.upper);
    content = content.replace("{TABLE_L}", &table.lower);
    return content;
}

/**Function to geenerate Folder Structure */
pub fn gen_structure(base:&str, callback:fn()){
    let folders: Vec<String> = vec![
        "controllers".to_string(),
        "models".to_string(),
        "resources".to_string(),
        "route".to_string(),
    ];

    for folder in folders {
        let dist = &format!("{0}\\{1}",base,folder);
        fs::create_dir_all(dist).unwrap();
    }
    callback();

}