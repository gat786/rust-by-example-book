#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let user = User {
        email: String::from("ganesht049@gmail.com"),
        username: String::from("ganesht049"),
        active: true,
        sign_in_count: 1,
    };

    println!("User: {}", user.email);

    let mut mutableUser = User {
        email: String::from("gat786@outlook.com"),
        username: String::from("gat786"),
        active: true,
        sign_in_count: 100,
    };

    println!("User: {:?}", mutableUser);
    mutableUser.email = String::from("gat786@someotherdomain.com");
    println!("Updated user: {:?}", mutableUser);

    let user2 = build_user(String::from("vaibhav@gmail.com"), String::from("vaibhav"));
    println!("User2: {:?}", user2);

    let user3 = build_user_shorthand(String::from("rahul@gmail.com"), String::from("rahul"));
    println!("User3: {:?}", user3);

    let user4 = build_user_from_other_user(user3);
    println!("User4: {:?}", user4);

    // Initialising tuple structs
    let black = Color(0, 0, 0);
    let orange = Color(255, 165, 0);
    let red = Color(255, 0, 0);

    let origin = Point(0, 0, 0);
    let x_axis = Point(1, 0, 0);
    let y_axis = Point(0, 1, 0);

}

// initialize a struct
fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}

// initialize a struct using field init shorthand
fn build_user_shorthand(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
} 

// create a struct from another struct
fn build_user_from_other_user(user: User) -> User {
    User {
        email: user.email,
        username: user.username,
        active: user.active,
        sign_in_count: user.sign_in_count,
    }
}

// example of tuple structs
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
