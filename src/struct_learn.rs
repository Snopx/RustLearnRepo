use std::u32;

pub fn struct_learn() {
    let mut user = User {
        username: String::from("Darren"),
        email: String::from("darren@gmail.com"),
        active: true,
        sign_in_count: 123,
    }; //使用这种方式，必须对每个属性赋值
    println!("{}", user.email);
    //获取/赋值 struct 中的值 赋值操作必须是可变的
    user.email = String::from("Snopx@gmail.com");
    println!("{}", user.email);

    //一旦 struct 声明时是可变的 ，那么其中所有字段都是可变的，不允许某一部分属性不可变

    let user1 = build_user("user1", "@");
    println!("{},{}", user1.email, user1.username);

    let user2 = build_user2(String::from("user2"), String::from("@"));
    println!("{},{}", user2.email, user2.username);

    let user3 = User {
        username: "user3".to_string(),
        ..user2 //当属性中的某些值与之前的 struct 相同是 可使用该方式简写
    };
    println!("{},{}", user3.email, user3.username);
}

pub struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(username: &str, email: &str) -> User {
    User {
        email: email.to_string(),
        username: username.to_string(),
        sign_in_count: 1,
        active: true,
    }
}
//变量名与属性名一样时可以简写tring
fn build_user2(username: String, email: String) -> User {
    User {
        email,
        username,
        sign_in_count: 1,
        active: true,
    }
}

// tuple struct:用于想给 tuple起名，而不想对其中的元素起名

//RGB
struct Color(i32, i32, i32);
//XYZ
struct Point(i32, i32, i32);

// 不同的类型 ，不同的 tuple struct 的实例；

// Unit-Like Struct ：适用于 需要在某个类型上实现某个trait（特性），但是里面又不存储数据
struct UnitList {}

// struct 数据所有权
//例如上方的 User struct ,其中属性使用的 'String' 而不是&str(字符串切片)
//该实例拥有其所有的数据
//只要struct是有效的，那么里面的字段数据也是有效的
//note: struct里也可以存放引用，但是需要使用生命周期(ref 之后的 生命周期中详解 life-time)。
//生命周期保证只要 struct 实例是有效的，那么里面的引用也是有效的
//如果struct 里存储引用，而不是用生命周期就会报错 如下

pub struct UserRef {
    //username: &str, // missing lifetime specifier
    //email: &str,
    sign_in_count: u64,
    active: bool,
}
