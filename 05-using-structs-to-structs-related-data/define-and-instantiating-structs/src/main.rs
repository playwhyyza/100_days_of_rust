struct Color(i32, i32, i32);
struct Point(i32, i32, i32);


struct AlwaysEqual;

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64
}

fn build_user(email: String, username: String) -> User {
    // User {
    //     active: true,
    //     username: username,
    //     email: email,
    //     sign_in_count: 1,
    // }

    /*
        field init shorthand syntax 
        because the parameter names and the struct field name are exactly the same
    */

    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");
    println!(
        "user1 -> username: {2} | email: {1} | active: {0} | sign_in_count: {3}", 
        user1.active, user1.email, user1.username, user1.sign_in_count
    );

    let user2 = build_user(
        String::from("someone@example.com"),
        String::from("someusername123"),
    );
    println!(
        "user2 -> username: {2} | email: {1} | active: {0} | sign_in_count: {3}", 
        user2.active, user2.email, user2.username, user2.sign_in_count
    );

    // creating instances from other instances with struct update syntax
    let user3 = User {
        active: user1.active,
        username: user2.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };
    println!(
        "user3 -> username: {2} | email: {1} | active: {0} | sign_in_count: {3}", 
        user3.active, user3.email, user3.username, user3.sign_in_count
    );

    // using struct update syntax
    let user4 = User {
        email: String::from("another@example.com"),
        ..user1 // the syntax .. specifies that the remaining fields not explicitly set should have the same value as the fields in the given instance.
    };
    println!(
        "user4 -> username: {2} | email: {1} | active: {0} | sign_in_count: {3}", 
        user4.active, user4.email, user4.username, user4.sign_in_count
    );

    // Using Tuple Structs without Named Fields to Create Different Types
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    println!("black: {0}", black.0); // use . followed by the index to access an individual value
    println!("origin: {0}", origin.1);

    let subject = AlwaysEqual;
}