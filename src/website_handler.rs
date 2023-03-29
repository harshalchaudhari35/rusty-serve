use crate::http::StatusCode;

use super::server::Handler;
use super::http::{Request, Response, Method};
use std::fs;
pub struct WebsiteHandler {
    public_path: String
}

impl WebsiteHandler {
    pub fn new(public_path: String) -> Self {
        Self { public_path }
    }

    fn read_file(&self, file_path: &str) -> Option<String> {
        let path = format!("{}/{}", self.public_path, file_path);

        match fs::canonicalize(path) { // resolve symlinks and get absolute path
            Ok(path) => {
                if path.starts_with(&self.public_path) { // only get files paths under public_path
                    fs::read_to_string(path).ok() // read file to string and convert Result to option using ok()
                } else {
                    println!("Directory Traversal Attack Attempted: {}", file_path);
                    None
                }
            }
            Err(_) => None,
        }
    }
}

impl Handler for WebsiteHandler {
    fn handle_request(&mut self, request: &Request) -> Response {
        // Response::new(StatusCode::OK, Some("<h1>TEST</h1>".to_string()))

        match request.method() {
            Method::GET => match request.path() {
                "/" => Response::new(StatusCode::OK, self.read_file("index.html")),
                "/hello" => Response::new(StatusCode::OK, self.read_file("hello.html")),
                path => match self.read_file(path) {
                    Some(contents) => Response::new(StatusCode::OK, Some(contents)),
                    None => Response::new(StatusCode::NotFound, None),
                }
            }
            _ => Response::new(StatusCode::NotFound, None),

        }
    }

    // fn handle_bad_request(&mut self, e: &std::string::ParseError) -> crate::http::Response {

    // }
}