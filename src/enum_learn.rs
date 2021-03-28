pub fn enum_learn() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    route(four);
    route(six);
    route(IpAddrKind::V4);

    let ipa = Ipaddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let ip4 = IpAddrKind2::V4(127, 0, 0, 1);
    let ip6 = IpAddrKind2::V6(String::from("::1"));

    let m = Message::Move { x: 12, y: 12 };
    m.call();

    //Option<T> 枚举
    let some_number = Some(1);
    let some_string = Some("A string");
    let none: Option<i32> = None;
    //assert_eq!(none.is_some(), true, "none is not Option::None");

    let x: i8 = 12;//
    let y = Some(5);
    let sum = x + y.unwrap();//相加值超过 127 报错
    println!("x+y={}", sum);
}

enum IpAddrKind {
    V4,
    V6,
}

fn route(ipk: IpAddrKind) {}

struct Ipaddr {
    kind: IpAddrKind,
    address: String,
}
//将数据附加到枚举变体中
enum IpAddrKind2 {
    V4(u8, u8, u8, u8),
    V6(String),
}
#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(u32, u32, u32),
}

//为枚举定义方法
impl Message {
    fn call(&self) {
        println!("{:#?}", self);
    }
}
