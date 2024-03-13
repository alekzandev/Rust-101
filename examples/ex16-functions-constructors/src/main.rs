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
    fn from_email(&mut self, email: &str) -> Self {
        let vec_username = email.split("@").collect::<Vec<&str>>();
        let username = vec_username[0].to_string();
        let uri = self.uri.to_string();
        Self {
            username: username,
            email: email.to_string(),
            uri: uri,
            active: true
        }
    }
    fn update_uri(&mut self, uri: &str){
        self.uri = uri.to_string()
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

    let mut user3 = user2.from_email("john@doe.com");
    println!("The user {} is active = {}, with email {}, and uri {}.", user3.username, user3.active, user3.email, user3.uri);
    user3.update_uri("http://example.com/john");
    println!("The user {} is active = {}, with email {}, and uri {}.", user3.username, user3.active, user3.email, user3.uri);
}