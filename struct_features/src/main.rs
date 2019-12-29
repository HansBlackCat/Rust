

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool
}

fn main() {
    let user1 = User {
        email: String::from("abc@example.com"),
        username: String::from("NaNaNa"),
        active: true,
        sign_in_count: 1
    };

    let mut user2 = User {
        email: String::from("abc@example.com"),
        username: String::from("NaNaNa"),
        active: true,
        sign_in_count: 1
    };
    user2.active = false;

    let user3 = User {
        email: String::from("abc@example.com"),
        username: String::from("NaNaNa"),
        ..user2
    };
}

fn user_maker_with_default(username: String, email: String) ->User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1
    }
}
