use std::{collections::HashMap, io::{BufRead, BufReader, Read}, str::FromStr};
use once_cell::sync::Lazy;

#[derive(Debug, Clone)]
pub struct HTTPRequest {
    pub request_line: RequestLine,
    pub headers: HTTPHeaders,
    pub body: Option<String>,
}

#[derive(Debug, Clone)]
pub struct RequestLine {
    pub method: String,
    pub uri: String,
    pub http_version: String,
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
        let method: String = iterator
            .next()
            .ok_or("Failed to get HTTP method")?
            .to_string();
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

impl HTTPHeaders {
    pub fn new(iterator: &mut impl Iterator<Item = String>) -> Result<Self, String> {
        let mut headers = HashMap::new();
        for line in iterator {
            if line.is_empty() {
                break;
            }
            // println!("line: {}", line);

            let mut line = line.split(':');
            let key = line.next().ok_or("Failed to get key")?.to_string();
            
            let is_host_header = if key == "Host" {
                true
            } else {
                false
            };

            let value = line
                .next()
                .ok_or(format!("Failed to get value for key: {key}"))?
                
                .to_string();

            if is_host_header {
                // Handle Host header which is a special case since
                // it contains ':'. So split() won't work as expected.
                let port = line
                    .next()
                    .ok_or(format!("Failet to get port on host header"))?
                    .to_string();    
                
                let complete_value = format!("{}:{}", value, port);
                headers.insert(key, complete_value);
                continue;
            }

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
pub struct HTTPResponse {
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    /// Helper method for testing
    fn make_reader(lines: &[&str]) -> BufReader<Cursor<Vec<u8>>> {
        let joined = lines.join("\r\n").into_bytes();
        BufReader::new(Cursor::new(joined))
    }

    #[test]
    fn parse_request_simple_get() {
        let reader = make_reader(&[
            "GET /uri HTTP/1.1",
            "Host: mockhost:2020",
            "User-Agent: mock_agent/0.0.0",
            "Accept: */*",
            "",
        ]);

        let req = HTTPRequest::try_from(reader).expect("parsing GET");
        assert_eq!(req.request_line.method, String::from("GET"));
        assert_eq!(req.request_line.uri, "/uri");
        assert_eq!(req.request_line.http_version, "HTTP/1.1");

        // headers
        assert_eq!(
            req.headers.0.get("Host").map(String::as_str),
            Some(" mockhost:2020")
        );
        // empty body
        assert!(req.body.is_none());
    }

    #[test]
    fn parse_request_post_with_body() {
        let body = r#"field=valor&otro=2"#;
        let content_length = body.len();
        let reader = make_reader(&[
            "POST /form HTTP/1.1",
            "Host: 127.0.0.1:2020",
            &format!("Content-Length: {content_length}"),
            "",
            body,
        ]);

        let req = HTTPRequest::try_from(reader).expect("parsing POST");

        assert_eq!(req.request_line.method, String::from("POST"));
        assert_eq!(req.body.as_deref(), Some(body));
    }

    #[test]
    fn request_invalid_method_fails() {
        let reader = make_reader(&["BREW / HTTP/1.1", "", ""]);
        assert!(HTTPRequest::try_from(reader).is_err());
    }

    // ------------ response tests ------------

    #[test]
    fn parse_response_ok() {
        let reader = make_reader(&[
            "HTTP/1.1 200 Ok",
            "Content-Type: text/plain",
            "",
            "hola",
        ]);

        let res = HTTPResponse::try_from(reader).expect("parsing response");

        assert_eq!(res.status_line.http_version, "HTTP/1.1");
        assert_eq!(res.status_line.status_code.0, 200);
        assert_eq!(res.status_line.status_data, "Ok");
        assert_eq!(res.body.as_deref(), Some("hola"));
    }

    #[test]
    fn response_without_body() {
        let reader = make_reader(&[
            "HTTP/1.1 204 No-Content",
            "Date: hoy",
            "",
        ]);

        let res = HTTPResponse::try_from(reader).expect("parsing 204");
        assert!(res.body.is_none());
    }
}