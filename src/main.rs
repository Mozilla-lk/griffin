use serde_derive::Deserialize;
use std::env;
use std::fs;

#[derive(Deserialize)]
pub struct Configuration{
    server_name: String,
    ip: String   
}

fn main() {
    println!("Hello, world!");
    
    // Read config.toml and show output
    let contents = fs::read_to_string("config.toml")
        .expect("Something went wrong reading the file");

    println!("With text:\n{}", contents);

}
