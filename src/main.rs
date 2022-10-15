enum IpAddrKind {
    V4,
    V6,
}

struct IpAdress {
    adressType: IpAddrKind,
    adress: String,
}

enum IppAddrWithType {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quite,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

struct QuitMessage; // unit struct
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // tuple struct
struct ChangeColorMessage(i32, i32, i32); // tuple struct

enum Option<T> {
    None,
    Some(T),
}

fn main() {
    let home = IppAddrWithType::V4(127, 0, 0, 1);
    let localhost = IpAdress {
        adressType: IpAddrKind::V4,
        adress: String::from("127.0.0.1"),
    };
    println!("{}", localhost.adress);
    let m = Message::Write(String::from("Hello world!"));
    m.call();
    let some_char = Some("H");
    let some_num = Some(12);
    // let abs: Option<i32> = None;
    let val = get_currency_in_dinar(500.0 , Currency::Dinar);
    println!("{}" , val)
}

impl Message {
    fn call(&self) {
        println!("Inside body function!!  ");
    }
}

enum Currency {
    Dinar,
    Dollar,
    Euro,
}

fn get_currency_in_dinar(value: f32, currency: Currency) -> f32{
    match(currency) {
        Currency::Dinar => value,
        Currency::Dollar => value * 221.00,
        Currency::Euro => value * 214.00
    }
}
