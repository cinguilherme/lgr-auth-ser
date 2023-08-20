#![allow(dead_code, unused_variables)]

use rand::prelude::*;

mod auth_utils;
mod database;

use database::*;

pub use auth_utils::models::Credentials;

pub fn authenticate(creds: Credentials) {
    let backoff = thread_rng().gen_range(100..500);
    println!("the backoff time is {backoff} ms");

    if let Status::Connected = connect_to_database() {
        auth_utils::login(creds)
    } else {
        println!("Could not connect to database");
    }
}
