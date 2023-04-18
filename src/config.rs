
use std::{env};

pub fn get_public_path() -> String {
    let default_path = format!("{}/public", env!("CARGO_MANIFEST_DIR"));
    let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);
    println!("{}", public_path);
    public_path
}