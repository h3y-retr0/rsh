pub mod server;
pub mod parser;
pub mod db;

use self::server::{Server, Config};
use self::db::Storage;
fn main() {
    let host = std::net::IpAddr::V4(std::net::Ipv4Addr::new(127, 0, 0, 1));
    let port = 8080;
    let storage = Storage::new();

    let conf = Config { host, port, storage};

    let mut server = Server::new(conf);

    server.run().unwrap();
    
}



