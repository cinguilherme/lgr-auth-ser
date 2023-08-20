use auth_service::{authenticate, Credentials};

fn main() {
    let creds = Credentials {
        username: String::from("test"),
        password: String::from("test"),
    };

    authenticate(creds);
}
