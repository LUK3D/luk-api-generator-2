use std::fs;
#[path="luk_utils.rs"]
mod luk_utils;
use luk_utils::read_file;
/**Generate Routes */
pub fn gen_routers(){

    let mut content = read_file("./templates/route.luk");

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