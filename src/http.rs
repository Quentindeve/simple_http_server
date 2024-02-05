/// All supported HTTP methods
#[derive(Debug)]
pub enum HttpMethod {
    Get,
}

#[derive(Debug)]
pub enum HttpProtocolVersion {
    Http11,
}

#[derive(Debug)]
pub struct HttpRequest {
    pub protocol_version: HttpProtocolVersion,
    pub method: HttpMethod,
    pub route: String,
}

#[derive(Copy, Clone)]
pub enum HttpStatus {
    Ok = 200,
    NotFound = 404,
}

pub enum HttpContentType {
    Html,
}

#[derive(Debug)]
pub enum HttpRequestParseError {
    InvalidUtf8Buffer,
    InvalidMethod,
    InvalidProtocolHeader,
    UnsupportedHttpVersion(String),
}

impl std::fmt::Display for HttpRequestParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for HttpRequestParseError {}

const LINE_BREAK: &str = "\r\n";

impl TryFrom<&[u8; 2048]> for HttpRequest {
    type Error = HttpRequestParseError;

    fn try_from(value: &[u8; 2048]) -> Result<Self, Self::Error> {
        let buf =
            std::str::from_utf8(value).map_err(|_| HttpRequestParseError::InvalidUtf8Buffer)?;

        parse_first_line(buf)
    }
}

fn parse_first_line(text: &str) -> Result<HttpRequest, HttpRequestParseError> {
    let split: Vec<&str> = text.split(" ").collect();
    if split.len() < 3 {
        return Err(HttpRequestParseError::InvalidProtocolHeader);
    }
    let method = split[0];

    let method = match method {
        "GET" => Ok(HttpMethod::Get),
        _ => Err(HttpRequestParseError::InvalidMethod),
    }?;

    let route = split[1].replace("+", " ");

    let route = if route == "/" {
        String::from("index.html")
    } else {
        if route.starts_with("/") {
            let mut iter = route.chars();
            let _ = iter.next();
            String::from(iter.as_str())
        } else {
            route
        }
    };

    let protocol = split[2].split("\r\n").nth(0);
    let protocol = match protocol {
        Some("HTTP/1.1") => Ok(HttpProtocolVersion::Http11),
        _ => Err(HttpRequestParseError::UnsupportedHttpVersion(String::from(
            split[2],
        ))),
    }?;

    Ok(HttpRequest {
        protocol_version: protocol,
        method,
        route,
    })
}

pub struct HttpResponse {
    protocol: HttpProtocolVersion,
    status: HttpStatus,
    content_type: HttpContentType,
    body: String,
}

impl HttpResponse {
    pub fn new(
        protocol: HttpProtocolVersion,
        status: HttpStatus,
        content_type: HttpContentType,
        body: String,
    ) -> Self {
        Self {
            protocol,
            status,
            content_type,
            body,
        }
    }

    pub fn build(&self) -> Vec<u8> {
        format!(
            "HTTP/1.1 {} OK\nContent-Type: {}\r\nContent-Length: {}\r\n\r\n{}", // TODO: Replace "OK" with the appropriate message
            self.status as u32,
            "text/html", // TODO: Not (alwyas) text/html
            self.body.len(),
            self.body
        )
        .bytes()
        .collect()
    }
}
