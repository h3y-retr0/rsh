use mysql::{*, prelude::*};
use std::env;
use dotenv::dotenv;

use crate::parser::Method;

pub struct Storage {
    conn: PooledConn,
}

pub struct Data {
    method: Method,
    content: String,
}

impl Storage {
    pub fn new() -> Self {
        dotenv().ok();
    
        let db_name = env::var("DB_NAME").expect("DB name not specified");
        let db_username = env::var("DB_USERNAME").expect("DB username not specified");
        let db_password = env::var("DB_PASSWORD").expect("DB password not specified");
        
        let url = format!("mysql://{}:{}@localhost:3306/{}", db_username, db_password, db_name);
        let pool = Pool::new(url.as_str()).unwrap();
        let conn = pool.get_conn().unwrap();

        Storage { conn }
    }

    pub fn start_db(&mut self) -> std::io::Result<()> {
        self.conn.query_drop(
            r"CREATE TABLE IF NOT EXISTS resources (
                id INT AUTO_INCREMENT PRIMARY KEY,
                path VARCHAR(255) NOT NULL,
                method ENUM('GET', 'POST', 'PUT', 'DELETE') NOT NULL,
                content MEDIUMTEXT,
                UNIQUE KEY unique_path_method (path, method)
            );"
        ).unwrap();

        Ok(())
    }

    pub fn find(&mut self, method: String, uri: String) -> Option<String> {

        let tuple = self.conn.exec_first(
            "SELECT content FROM resources WHERE method = ? AND path = ?", 
            (method, uri),
        ).unwrap_or(None);

        tuple
    }
}



    


  


