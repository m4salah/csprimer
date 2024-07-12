use std::{collections::HashMap, str::FromStr};

#[derive(Debug)]
pub struct HTTPRequest {
    pub method: String,
    pub path: String,
    pub version: String,
    pub headers: HashMap<String, String>,
    pub body: String,
}

#[derive(Debug)]
pub enum HTTPRequestParseError {
    ParseHTTPError(String),
}

impl FromStr for HTTPRequest {
    type Err = HTTPRequestParseError;

    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        println!("{s}");
        let mut sections = s.split("\r\n\r\n");

        let mut http_header = sections
            .next()
            .and_then(|meta_section| Some(meta_section.split("\r\n")))
            .ok_or(HTTPRequestParseError::ParseHTTPError(
                "Failed to parse HTTP Header".to_string(),
            ))?;

        let mut meta_section = http_header
            .next()
            .and_then(|meta_section| Some(meta_section.split(" ")))
            .ok_or(HTTPRequestParseError::ParseHTTPError(
                "Failed to parse Meta Header".to_string(),
            ))?;

        let method = meta_section
            .next()
            .ok_or(HTTPRequestParseError::ParseHTTPError(
                "Failed to parse the method".to_string(),
            ))?
            .to_string();

        let path = meta_section
            .next()
            .ok_or(HTTPRequestParseError::ParseHTTPError(
                "Failed to parse the path".to_string(),
            ))?
            .to_string();

        let version = meta_section
            .next()
            .ok_or(HTTPRequestParseError::ParseHTTPError(
                "Failed to parse the version".to_string(),
            ))?
            .to_string();

        let mut headers = HashMap::new();
        for header in http_header {
            let (key, value) =
                header
                    .split_once(": ")
                    .ok_or(HTTPRequestParseError::ParseHTTPError(
                        "Failed to parse the actual header".to_string(),
                    ))?;

            headers.insert(key.to_string(), value.to_string());
        }

        let body = sections
            .next()
            .ok_or(HTTPRequestParseError::ParseHTTPError(
                "Failed to parse the body".to_string(),
            ))?
            .to_string();

        Ok(HTTPRequest {
            method,
            path,
            version,
            headers,
            body,
        })
    }
}
