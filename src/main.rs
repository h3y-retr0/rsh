pub mod server;

use self::server::{Server, Config};

fn main() {
    let ip = std::net::IpAddr::V4(std::net::Ipv4Addr::new(127, 0, 0, 1));

    let conf = Config { host: ip, port: 8080, };

    let server = Server::new(conf);

    // match server.run() {
    //     Ok(_) => println!("Server listening on port {}", server.config.port),
    //     Err(e) => println!("Error {}", e),
    // };
    server.run().unwrap();
}



