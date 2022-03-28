fn main() {
    let mut user1 = User {
        email: String::from("john.doe@mail.org"),
        username: String::from("john-o-doe"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("other.mail@mail.org");

    let user2 = build_user(String::from("some@mail.org"), String::from("someuser"));
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}