struct User {
    name: String,
    email: String,
    active: bool,
}

fn main() {
    let user = User {
        name: String::from("홍길동"),
        email: String::from("gildong@example.com"),
        active: true,
    };

    println!("name = {}", user.name);
}

fn struct_mut() {
    let mut user = User {
        name: String::from("홍길동"),
        email: String::from("gildong@example.com"),
        active: true,
    };

    user.email = String::from("gd.hong@example.com");
}

fn build_user(name: String, email: String) -> User {
    User {
        name: name,
        email: email,
        active: true,
    }
}

fn build_user_short(name: String, email: String) -> User {
    User {
        name,
        email,
        active: true,
    }
}

fn another_struct() {
    let user1 = User {
        name: String::from("홍길동"),
        email: String::from("gildong@example.com"),
        active: true,
    };

    let user2 = User {
        name: user1.name,
        email: user1.email,
        active: false,
    };
}

fn another_struct_short() {
    let user1 = User {
        name: String::from("홍길동"),
        email: String::from("gildong@example.com"),
        active: true,
    };

    let user2 = User {
        active: false,
        ..user1
    };
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn tuple_struct() {
    let white = Color(255, 255, 255);
    let origin = Point(0, 0, 0);
}
