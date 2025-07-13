pub mod server;
pub mod parser;
pub mod db;

use self::server::{Server, Config};
use self::db::*;
fn main() {
    let host = std::net::IpAddr::V4(std::net::Ipv4Addr::new(127, 0, 0, 1));
    let port = 8080;

    let conf = Config { host, port, };

    let server = Server::new(conf);

    start_db().expect("Error while trying to connect to mysql instance");
    println!("DB Connected");

    server.run().unwrap();
    
}



