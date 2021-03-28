pub fn match_learn() {
    let c = Coin::Quarter(UsState::Alabama);
    let money = value_in_cents(c);

    println!("quarter money is {}",money);

    //match option<T>
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    // rust match 必须所有情况 可以使用 _ 下划线通配符表示其他情况
    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => println!("others"),
    }

    let some_u8_value = Some(3u8);
    // match 不会转换所有权
    match some_u8_value {
        Some(3) => println!("three"),
        _ => (),
    }
    // 👆 == 👇
    if let Some(3) = some_u8_value {
        println!("three");
    }
    if let Some(4) = some_u8_value {
        println!("no three");
    }else{
        println!("no three");
    }

}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(s) => {
            println!("State quarter from {:#?}",s);
            25
        },
    }
}

#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
