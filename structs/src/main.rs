struct User{
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool
}


fn main() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someoneusername123"),
        active: true,
        sign_in_count: 1,
    };
    let user2 = build_user(String::from("someonetwo@gmail.com"), String::from("someonetwousername"));
    let user3 = User {
        email: String::from("someonethree@exmaple.com"),
        username: String::from("someonethreeusername"),
        ..user2
    };



}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
