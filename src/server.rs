use std::net::{IpAddr, TcpListener, TcpStream};
use std::io::{Read, Write};
use std::collections::HashMap;
use once_cell::sync::Lazy;


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

// #[repr(u16)]
// #[derive(Debug)]
// pub enum HttpStatusCode {
//     Continue = 100,
//     Ok = 200,
//     BadRequest = 400,
//     Unauthorized = 401,
//     NotFound = 404,
//     InternalServerError = 500,
    
//     Other(u16),
// }

// impl From<u16> for HttpStatusCode {
//     fn from (status_code: u16) -> Self {
//         match status_code {
//             100 => Self::Continue,
//             200 => Self::Ok,
//             400 => Self::BadRequest,
//             401 => Self::Unauthorized,
//             404 => Self::NotFound,
//             500 => Self::InternalServerError,
//             _ => Self::Other(status_code),
            
//         }
//     }
// }

static HTTP_STATUS_CODE: Lazy<HashMap<u16, &'static str>> = Lazy::new(|| {
    HashMap::from([
        (100, "Continue"),
        (200, "Ok"),
        (400, "Bad Request"),
        (401, "Unauthorized"),
        (404, "Not Found"),
        (500, "Internal Server Error"),
    ])
});



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

    fn response_line(&self, status_code: u16) -> Option<String> {
        // There might be a better implementation of this.
        if let Some(txt) = HTTP_STATUS_CODE.get(&status_code) {
            return Some(format!("HTTP/1.1 {} {}\r\n", status_code, txt));
        }

        None
    }

    /// Creates a new HTTP response.
    /// An HTTP response has the following format:
    /// 
    /// HTTP/1.1 200 OK -> Response line
    /// --------------- -> Response header
    /// --------------- -> Response header
    /// --------------- -> Response header
    ///                 -> Blank line
    /// <html><h1>Response body</h1></html>
    /// 
    /// Note that the \r\n characters are line break characters. They are present
    /// at the end of every line in an HTTP response except on the body.
    fn create_response(&self, _data: Option<String>) -> Option<String>{
        let s_code = 200;
        let line = self.response_line(s_code);
        let status_line = if line.is_some() {
            line.unwrap()
        } else {
            format!("HTTP/1.1 {} {}\r\n", 500, "Internal Server Error")
        };

        let body = "<html><body><h1>Request received!</h1></body></html>";

        let headers = format!(
            concat!(
                "Content-Type: text/html\r\n",
                "Content-Length: {}\r\n",
                "Connection: close\r\n\r\n"
            ),
            body.as_bytes().len()
        );

        let blank_line = "\r\n";

        Some(format!("{status_line}{headers}{blank_line}{body}"))
    }
}