#![allow(dead_code, unused_variables)]

pub struct Credentials {
    username: String,
    password: String,
}

pub enum Status {
    Connected,
    Interrupted,
}

fn connect_to_database() -> Status {
    return Status::Connected;
}

fn login(creds: Credentials) {
    println!("Logging in with username: {}", creds.username);
}

fn logout() {
    println!("Logging out");
}

fn get_user() {}

pub fn authenticate(creds: Credentials) {
    if let Status::Connected = connect_to_database() {
        login(creds)
    } else {
        println!("Could not connect to database");
    }
}
