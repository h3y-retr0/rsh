pub mod server;
pub mod parser;

use self::server::{Server, Config};

fn main() {
    let host = std::net::IpAddr::V4(std::net::Ipv4Addr::new(127, 0, 0, 1));
    let port = 8080;

    let conf = Config { host, port, };

    let server = Server::new(conf);

    server.run().unwrap();
}



