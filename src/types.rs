
pub struct Dados {
    pub table_name: String
}


/** ## Database Configuration */
pub struct DbConfig{
    /**Username */
    pub user:String,
    /**Database */
    pub db:String,
    /**Password */
    pub pwd:String,
    /**Host */
    pub host:String
}


pub struct TableName{
    pub lower:String,
    pub upper:String,
    pub columns: Vec<String>
}

pub struct TableInfo{
    pub names:Vec<TableName>
}