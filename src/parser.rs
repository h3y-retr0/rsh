use std::{collections::HashMap, io::{BufRead, BufReader, Read}, str::FromStr};

use once_cell::sync::Lazy;


/// Very naive and basic implementation of an HTTP parser.
// pub struct Parser<'a> {
//     method: Option<&'a str>,
//     uri: Option<&'a str>,
//     http_version: &'a str,
//     data: Option<&'a str>,
// }

#[derive(Debug, Clone)]
pub struct HTTPRequest {
    request_line: RequestLine,
    headers: HTTPHeaders,
    body: Option<String>,
}

#[derive(Debug, Clone)]
pub struct RequestLine {
    method: Method,
    uri: String,
    http_version: String,
}

#[derive(Debug, Clone)]
struct HTTPHeaders(HashMap<String, String>);

#[derive(Debug, Clone)]
pub enum Method {
    GET,
    POST,
    HEAD,
    OPTIONS,
    DELETE,
    PUT,
    CONNECT,
    TRACE,
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

/// An HTPP Request has the following structure we need to handle:
/// METHOD uri HTTP_VERSION -> Request line
/// ----------------------- |
/// ----------------------- |
/// ----------------------- | -> Request headers 
/// ----------------------- |
/// ----------------------- |
///                         -> Blank line that separates Header & Body
/// Request message body
/// 
/// 

impl FromStr for RequestLine {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iterator = s.split(' ');
        let method: Method = iterator
            .next()
            .ok_or("Failed to get HTTP method")?
            .parse()?;
        let uri = iterator
            .next()
            .ok_or("Failed to get requet uri")?
            .to_string();
        let http_version = iterator
            .next()
            .ok_or("Failed to get HTTP version")?
            .to_string();
        
        Ok(RequestLine { method, uri, http_version })
    }
}

impl FromStr for Method {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            // do we need to handle HTTP methods in lowercase?
            "GET" => Ok(Method::GET),
            "POST" => Ok(Method::POST),
            "PUT" | "put" => Ok(Method::PUT),
            "DELETE" => Ok(Method::DELETE),
            "HEAD" => Ok(Method::HEAD),
            "OPTIONS" => Ok(Method::OPTIONS),
            "CONNECT" => Ok(Method::CONNECT),
            "TRACE" => Ok(Method::TRACE),
            _ => Err(format!("invalid HTTP method: {s}")),
        }
    }
}

// impl<'a> Parser<'a> {
//     /// Creates and returns a new HTTP [`Parser`]
//     /// by default it will use 1.1 as the HTTP version.
//     pub fn new(data: &'a str) -> Self {
//         Self {
//             method: None,
//             uri: None,
//             http_version: "1.1",
//             data: Some(data),
//         }
//     }

//     /// Parses the HTTP response.
//     fn parse(&'a mut self) -> Result<(), std::io::Error> {
        
//         let lines: Vec<&str> = self.data.as_ref().unwrap().split("\r\n").collect();
        
//         let request_line = lines[0];

//         let words: Vec<&str> = request_line.split(" ").collect();

//         self.method = Some(words[0]);

//         if words.len() > 1 {
//             self.uri = Some(words[1]);
//         }

//         if words.len() > 2 {
//             self.http_version = words[2];
//         }
        

//         Ok(())
//     }
// }

impl HTTPHeaders {
    pub fn new(iterator: &mut impl Iterator<Item = String>) -> Result<Self, String> {
        let mut headers = HashMap::new();
        for line in iterator {
            if line.is_empty() {
                break;
            }

            let mut line = line.split(':');
            let key = line.next().ok_or("Failed to get key")?.trim().to_string();
            let value = line
                .next()
                .ok_or(format!("Failed to get value for key: {key}"))?
                .to_string();
            headers.insert(key, value);
        }
        Ok(HTTPHeaders(headers))
    }
}

impl<R: Read> TryFrom<BufReader<R>> for HTTPRequest {
    type Error = String;

    fn try_from(value: BufReader<R>) -> Result<Self, Self::Error> {
        let mut iterator = value.lines().map_while(Result::ok).peekable();
        let request_line = iterator
            .next()
            .ok_or("Failed trying to get Request line")?
            .parse()?;
        let headers = HTTPHeaders::new(&mut iterator)?;
        let body = if iterator.peek().is_some() {
            Some(iterator.collect())
        } else {
            None
        };

        Ok(HTTPRequest { request_line, headers, body })
    }
}


#[derive(Debug, Clone)]
struct StatusCode(u16);

impl FromStr for StatusCode {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse::<u16>()
            .or(Err(format!("Error parsing the status code: {}", s)))
            .map(StatusCode)
    }
}

#[derive(Debug, Clone)]
struct StatusLine {
    // TODO: wrap http_version on a tuple struct HttpVersion(String)
    http_version: String,
    status_code: StatusCode,
    status_data: String,
}

impl FromStr for StatusLine {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iterator = s.split(' ');

        let http_version = iterator
            .next()
            .ok_or("Failed to get HTTP version")?
            .to_string();
        
        let status_code = iterator
            .next()
            .ok_or("Failed to get status code")?
            .parse()?;
        
        let status_data = iterator
            .next()
            .ok_or("Failed to get status data")?
            .to_string();
        
        Ok(StatusLine { http_version, status_code, status_data })
    }
}
#[derive(Debug, Clone)]
struct HTTPResponse {
    status_line: StatusLine,
    headers: HTTPHeaders,
    body: Option<String>,
}

impl<R: Read> TryFrom<BufReader<R>> for HTTPResponse {
    type Error = String;

    fn try_from(value: BufReader<R>) -> Result<Self, Self::Error> {
        let mut iterator = value.lines().map_while(Result::ok).peekable();
        let status_line: StatusLine = iterator
            .next()
            .ok_or("Failed to get status line")?
            .parse()?;
        
        let headers = HTTPHeaders::new(&mut iterator)?;

        let body = if iterator.peek().is_some() {
            Some(iterator.collect())
        } else {
            None
        };

        Ok(HTTPResponse { status_line, headers, body })
    }
}

