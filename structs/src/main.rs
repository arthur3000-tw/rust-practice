struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

struct AlwaysEqual;

// struct User2 {
//     active: bool,
//     username: &str,
//     email: &str,
//     sign_in_count: u64,
// }

fn main() {
    let mut user1 = User {
        active: true,
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        sign_in_count: 1,
    };
    user1.email = String::from("anotheremail@example.com");

    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };

    let user3 = User {
        email: String::from("another@example.com"),
        ..user2
    };

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    let subject = AlwaysEqual;

    // let user4 = User2 {
    //     active: true,
    //     username: "someusername123",
    //     email: "someone@example.com",
    //     sign_in_count: 1,
    // };
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        // email: email,
        email,
        // username: username,
        username,
        sign_in_count: 1,
    }
}