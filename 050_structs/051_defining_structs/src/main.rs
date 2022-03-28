fn main() {
    // structs
    {
        let mut user1 = User {
            email: String::from("john.doe@mail.org"),
            username: String::from("john-o-doe"),
            active: true,
            sign_in_count: 1,
        };

        user1.email = String::from("other.mail@mail.org");

        let user2 = build_user(String::from("some@mail.org"), String::from("someuser"));

        let user3 = User {
            email: String::from("userforlife@life.de"),
            ..user2
        };

        println!("{}", user3.username);
    }

    // tuple structs
    {
        let black = Color(0, 1, 2);
        let origin = Point(3, 2, 1);

        println!("{}", black.1);
        println!("{}", origin.0);
    }

    // unit-like structs
    {
        let subject = AlwaysEqual;
    }
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

// structs
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// tuple structs
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// unit-like structs
struct AlwaysEqual;
