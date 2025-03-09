#[derive(Debug)]
struct User {
    active:bool,
    username: String,
    email:String,
    sign_in_count: u64,
}

fn main() {
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    let mut user2 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    user2.active = false;

    let mut user3 =User {
        ..user1
    }; // user1 is no longer valid 

    println!("{:?}",user1.active);

    let mut user4 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: user1.sign_in_count
    };

    println!("{:?}",user4);

    let user5 = user4;

    // println!("{:?}",u);

    let Black = Color(0,0,0);
    let plane = Point(1,1,1);

}


fn make_User(email: String, active: bool) -> User {
    User {
        active,
        username : String::from("blah@gmail.com"),
        email,
        sign_in_count:100
    }
}

// there can also be tuple structs
struct Color(i32,i32,i32);
struct Point(i8,i8,i8);

// there can also be unit structs that can be used just to implement traits but not have any data in them
struct unitStruct;


// we want the struct to own its data as long as the struct is valid.