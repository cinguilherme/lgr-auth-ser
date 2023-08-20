use crate::database::get_user;

pub fn login(creds: models::Credentials) {
    println!("Logging in with username: {}", creds.username);
    get_user();
}

pub fn logout() {
    println!("Logging out");
}

pub mod models;
