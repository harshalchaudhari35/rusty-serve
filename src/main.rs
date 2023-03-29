#![allow(dead_code)]

use server::Server;
use website_handler::WebsiteHandler;
use std::{env};

mod website_handler;
mod server;
mod http;

fn main () {
    let default_path = format!("{}/public", env!("CARGO_MANIFEST_DIR"));
    let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);
    println!("{}", public_path);
    Server::new("127.0.0.1:8080".to_string()).run(WebsiteHandler::new(public_path));

}