
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1
    }
}


fn main () {

    let user1 = User {
        active: true,
        username: String::from("John Deer"),
        email: String::from("email@email.com"),
        sign_in_count: 1,
    };

    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("anotheremail@email.com"),
        sign_in_count: user1.sign_in_count, // equal to  email: String::from("anotheremail@email.com"), ..user1
    };

    
}