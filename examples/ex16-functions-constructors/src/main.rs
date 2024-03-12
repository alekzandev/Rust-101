struct User {
    username: String,
    email: String,
    uri: String,
    active: bool,
}

impl User {
    fn new(username: String, email: String, uri: String) -> Self {
        Self {
            username,
            email,
            uri,
            active: true,
        }
    }
    fn deactivate(&mut self) {
        self.active = false;
    }
}

fn main(){
    let user = User {
        username: String::from("user1"),
        email: String::from("user1@example.com"),
        uri: String::from("http://example.com/user1"),
        active: true,
    };
    println!("User: {}", user.username);

    let mut user2 = User::new(
        String::from("user2"),
        String::from("user2@example.com"),
        String::from("http://example.com/user2"),
    );
    println!("The {} is active = {}, with email {}, and uri {}.", user2.username, user2.active, user2.email, user2.uri);

    println!("User2 account is active?: {}", user2.active);

    user2.deactivate();
    println!("User2 account is active?: {}", user2.active);
}