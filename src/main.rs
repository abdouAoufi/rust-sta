struct User {
    name: String,
    is_logged: bool,
    age: i8,
    login_count: u64,
}

struct Color(i32, i32, i32);

fn main() {
    get_user();
    let user1 = build_user(String::from("Abdou"), 23);
    let user2: User = User {
        name: String::from("Abderahmane Aoufi"),
        ..user1
    };
    // println!("{:?}", user2.age);
    let color = Color(129, 141, 155);
    rectangle();
    let r1 = Rectangle {
        width : 33,
        height : 33
    };
    println!("{:?}" , r1);
    area2(&r1);
}

fn get_user() -> User {
    let full_name = String::from("Abderahamane Aoufi");
    User {
        name: full_name,
        is_logged: false,
        age: 23,
        login_count: 0,
    }
}

fn build_user(full_name: String, age: i8) -> User {
    User {
        name: full_name,
        age,
        login_count: 1,
        is_logged: true,
    }
}

fn area(dimenstion: (u32, u32)) -> u32 {
    dimenstion.0 * dimenstion.1
}
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn area2(dimension: &Rectangle) -> u32 {
    dimension.height * dimension.width
}

fn rectangle() {
    let width1: i32 = 450;
    let height1: i32 = 780;
    let d = (20, 50);
    println!("The area of the rectangle is {} square pixels.", area(d));
}
