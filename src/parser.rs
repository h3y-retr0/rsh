

/// Very naive and basic implementation of an HTTP parser.
pub struct Parser<'a> {
    method: Option<&'a str>,
    uri: Option<&'a str>,
    http_version: &'a str,
    data: Option<String>,
}

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
impl<'a> Parser<'a> {
    /// Creates and returns a new HTTP [`Parser`]
    /// by default it will use 1.1 as the HTTP version.
    pub fn new(data: String) -> Self {
        Self {
            method: None,
            uri: None,
            http_version: "1.1",
            data: Some(data),
        }
    }

    /// Parses the HTTP response.
    fn parse(&'a mut self) -> Result<(), std::io::Error> {
        
        let lines: Vec<&str> = self.data.as_ref().unwrap().split("\r\n").collect();
        
        let request_line = lines[0];

        let words: Vec<&str> = request_line.split(" ").collect();

        self.method = Some(words[0]);

        if words.len() > 1 {
            self.uri = Some(words[1]);
        }

        if words.len() > 2 {
            self.http_version = words[2];
        }
        

        Ok(())
    }
}