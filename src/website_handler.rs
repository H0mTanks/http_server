use std::fmt::format;
use std::fs;

use crate::http::{Method, Request, Response, StatusCode};
use crate::server::Handler;

pub struct WebsiteHandler {
    public_path: String,
}

impl WebsiteHandler {
    pub fn new(public_path: String) -> WebsiteHandler {
        WebsiteHandler { public_path }
    }

    fn read_file(&self, file_path: &str) -> Option<String> {
        let path = format!("{}/{}", self.public_path, file_path);
        fs::read_to_string(path).ok()
    }
}

impl Handler for WebsiteHandler {
    fn handle_request(&mut self, request: &Request) -> Response {
        match request.method() {
            Method::GET => match request.path() {
                "/" => Response::new(StatusCode::Ok, self.read_file("index.html")),
                "/hello" => Response::new(StatusCode::Ok, self.read_file("hello.html")),
                _ => Response::new(StatusCode::BadRequest, None),
            },
            _ => Response::new(StatusCode::BadRequest, None),
        }
    }
}
