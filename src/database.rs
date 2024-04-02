use std::env;


use diesel::prelude::*;
use dotenvy::dotenv;


pub fn establish_connection() -> MysqlConnection {
    // dotenv().ok();


    let database_url = "mysql://root:password@127.0.0.1:3306/mysql";


    MysqlConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Erro ao conectar no banco de dados {}", database_url))
}