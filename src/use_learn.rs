pub mod front_of_house_a {
    pub mod hosting {
        pub fn add_to_waitlist() {}
        fn some_fun() {}
    }
}

use front_of_house_a::hosting; // 惯用做法，mod 层级 不会指定到方法

pub fn eat_at() {
    hosting::add_to_waitlist();
}

// 针对 struct enum 等情况 指定到名称 如下方
use std::collections::HashMap;

//如果名称相同 则 引用到父级 👇
use std::fmt;
use std::io;
// fn f1() -> fmt::Result {
//     fmt::Result::ok();
// }

// fn f2() -> io::Result {
//     io::Result::ok();
// }

// pub use 可以对外暴露代码 如下
// pub use front_of_house_a::hosting; // 惯用做法，mod 层级 不会指定到方法

