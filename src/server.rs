use std::net::{IpAddr, TcpListener, TcpStream, SocketAddr};
use std::io::{BufRead, BufReader, Cursor, Write};
use std::collections::HashMap;
use std::time::SystemTime;
use once_cell::sync::Lazy;

use crate::db::Storage;
use crate::parser::{HTTPRequest, HTTPResponse};



pub(crate) struct Config {
    /// The IP address to bind to.
    pub host: IpAddr,
    /// The port to bind to.
    pub port: u16,
    /// The file storage we are using.
    pub storage: Storage,
}

pub(crate) struct Server {
    pub config: Config,
}

// Better way to do this?
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

    pub fn run(&mut self) -> Result<(), std::io::Error> {
        let addr = SocketAddr::from((self.config.host, self.config.port));
        let listener = TcpListener::bind(addr)?;
            

        self.config.storage.start_db().expect("Error while trying to connect to mysql instance");
        println!("DB Connected");
        println!("Server listening on http://{}", addr);

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => self.handle_conection(stream),
                Err(err) => eprintln!("Failed to accept connection: {err}"),
            }
        }
        
        Ok(())
    }
 

    fn handle_conection(&mut self, mut stream: TcpStream) {
        println!("New connection -> {:?}", stream);
    
        let unparsed_http_request: Vec<String> = BufReader::new(&stream)
            .lines()
            .map(|result| result.unwrap())
            .take_while(|line| !line.is_empty())
            .collect();
        
       
        
        // TODO: Refactor `parser` to use Vec<T> instead of BufRead<T>.
        let cur = Cursor::new(unparsed_http_request.join("\r\n").into_bytes());

        let reader = BufReader::new(cur);
        
        let req = HTTPRequest::try_from(reader).unwrap();

        if let Some(response) = self.create_response(req) {
            if let Err(e) = stream.write_all(response.as_bytes()) {
                eprintln!("Error writing to TCP Socket: {}", e);
                return;
            }
        }

        let _ = stream.flush();
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
    fn create_response(&mut self, request: HTTPRequest) -> Option<String>{
        // println!("{:?}", request);
        let method = request.request_line.method;
        let uri = request.request_line.uri;

        let db_result = self.config.storage.find(method, uri);

        let (status, body) = match db_result {
            Ok(Some(payload)) => (200, payload),
            Ok(None) => (404, "Not found".into()),
            Err(_e) => {
                // eprintln!("DB error: {e}");
                (500, "Internal server error".into())
            }
        };

        let content_type = "text/html";

        Some(self.build_response(status, &body, content_type))
    }

    fn build_response(&self, status: u16, body: &str, content_type: &str) -> String {
        let reason = HTTP_STATUS_CODE.get(&status).copied().unwrap_or("Unknown status");

        let date = httpdate::fmt_http_date(SystemTime::now());
        let body_len = body.as_bytes().len();

        format!(
            concat!(
                "HTTP/1.1 {} {}\r\n",
                "Date: {}\r\n",
                "Content-Type: {}\r\n",
                "Content-Length: {}\r\n",
                "Connection: close\r\n",
                "\r\n",
                "{}",
            ),
            status, reason, date, content_type, body_len, body
        )
    }
}

