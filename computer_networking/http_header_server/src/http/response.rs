use std::collections::HashMap;

#[derive(Debug)]
pub struct HTTPResponse {
    pub status_code: StatusCode,
    pub headers: HashMap<String, String>,
    pub body: String,
}

impl ToString for HTTPResponse {
    fn to_string(&self) -> String {
        let mut response_str = String::new();

        match &self.status_code {
            StatusCode::OK => {
                response_str.push_str("HTTP/1.1 200 OK\r\n");
            }
        }

        for (k, v) in self.headers.clone() {
            response_str.push_str(format!("{}: {}\r\n", k, v).as_str());
        }

        response_str.push_str("\r\n");
        response_str.push_str(&self.body);
        response_str.push_str("\r\n");

        return response_str;
    }
}

#[derive(Debug)]
pub enum ContentType {
    Json,
}

#[derive(Debug)]
pub enum StatusCode {
    OK,
}

impl HTTPResponse {
    pub fn new(status_code: StatusCode, content_type: ContentType, body: String) -> Self {
        let mut response_headers = HashMap::new();

        match content_type {
            ContentType::Json => {
                response_headers.insert("Content-Type".to_string(), "application/json".to_string());
            }
        };

        response_headers.insert("Content-Length".to_string(), body.bytes().len().to_string());
        Self {
            status_code,
            headers: response_headers,
            body,
        }
    }
}
