use std::net::{IpAddr, TcpListener, TcpStream};
use std::io::{Read, Write};


pub(crate) struct Config {
    /// The IP address to bind to.
    pub host: IpAddr,
    /// The port to bind to.
    pub port: u16,
}

pub(crate) struct Server {
    pub config: Config,
}

// struct Response<'a> {
//    body: &'a str,
//    headers: String,
//    content: Option<String>,
// }

impl Server {
    pub fn new(config: Config) -> Self {
        Server { config }
    }

    /// Returns the [`Server`] addr formated as host:port.
    pub fn get_server_addr(&self) -> String {
        format!("{}:{}", self.config.host, self.config.port)
    }

    pub fn run(&self) -> Result<(), std::io::Error> {
        let listener = TcpListener::bind(self.get_server_addr()).unwrap();
        println!("Server listening on http://{}", self.get_server_addr());
        for stream in listener.incoming() {
            let stream = stream.unwrap();
            
            self.handle_conection(stream); 
        }

        Ok(())
    }

    fn handle_conection(&self, mut stream: TcpStream) {
        println!("{:?}", stream);

        let mut buffer = [0u8; 1024];
        if let Err(e) = stream.read(&mut buffer) {
            eprintln!("Error reading the request: {e}");
            return;
        }

        if let Some(response) = self.create_response(None) {
            if let Err(e) = stream.write_all(response.as_bytes()) {
                eprintln!("Error writing to TCP Socket: {}", e);
                return;
            }
        }

        let _ = stream.flush();
    }

    fn create_response(&self, _data: Option<String>) -> Option<String>{
        let status_line = "HTTP/1.1 200 OK\r\n";

        let body = "<html><body><h1>Request received!</h1></body></html>";

        let headers = format!(
            concat!(
                "Content-Type: text/html\r\n",
                "Content-Length: {}\r\n",
                "Connection: close\r\n\r\n"
            ),
            body.as_bytes().len()
        );

        Some(format!("{status_line}{headers}{body}"))
    }
}