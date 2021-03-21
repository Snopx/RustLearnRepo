pub fn control_flow() {
    // if_statement();

    // if_muliple();

    if_to_match();

    let num = if true { 5 } else { 6 }; //必须使用相同类型
    println!("num={}", num);

    loop_method();
    while_method();

    for_method();

    for_count_down();
}

fn if_statement() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
}

fn if_muliple() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}

fn if_to_match() {
    let number = 6;
    match number {
        n if n % 4 == 0 => println!("number is divisible by 4"),
        n if n % 3 == 0 => println!("number is divisible by 3"),
        n if n % 2 == 0 => println!("number is divisible by 2"),
        _ => println!("number is not divisible by 4, 3, or 2"),
    }
}

fn loop_method() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2; //当 counter =10 时候 跳出且返回 20
        }
    };

    println!("The result is {}", result);
}

fn while_method() {
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("LIFTOFF!!!");
}

//遍历集合使用 for

fn for_method() {
    let a = [10, 20, 30];

    for i in &a {
        println!("i={}", i);
    }
}

// for 倒计时 (0..4) 1,2,3 ;(0..=4) 1 2 3 4;
fn for_count_down() {
    println!("----range count reverse to count down-----");
    for n in (1..=4).rev() {
        println!("{}", n);
    }
}
