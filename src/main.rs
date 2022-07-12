use reqwest::{header, ClientBuilder};
use anyhow::{Result};
use serde::{Serialize, Deserialize};
use serde_json;
use toml;

use std::fs::File;
use std::io::prelude::*;

fn read_addresses() -> std::io::Result<toml::Value> {
    let content = std::fs::read_to_string("addresses.toml")?;
    Ok(toml::from_str(&content)?)
}

fn main() {
    user_input1();

    let addresses = read_addresses().unwrap();

    println!("username: {}, token: {}", addresses["username"], addresses["token"]);
    println!("channel: {}, id: {}", addresses["channel"]["scotland_glasgow"]["name"], addresses["channel"]["scotland_glasgow"]["id"]);
}

fn user_input1() {
    let banner = std::fs::read_to_string("banner.txt")
        .expect("Could not read banner.txt");
    println!("{}", &banner);
}