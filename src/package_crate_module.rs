// 代码组织主要包括
// 那些细节可以暴露，哪些细节是私有的
// 作用域内哪些名称有效
// 模块系统
// Package ：cargo 特性，构建、测试、共享crate
// carge(单元包)： 一个模块数，它可以产生一个library或可执行文件
// Module（模块）、use： 控制代码组织，作用域，私有路径
// path: 为struct function module 等命名的方式

// crate的类型：
//      library
//      binary

// Crate Root:
// -是源代码文件
// Rust编译器从这里开始，组成你的Crate的根module

// 一个Package
// 包含一个Cargo.toml，他描述了如何构建这些Crates
// 只能包含0~1个 library crate
// 可以包含任意数量的 binary crate
// 但必须至少包含一个 crate （library,binary）

// Cargo 的惯例
// src/main.rs 是这个项目或者 package入口文件
// 此文件是 binary crate 的 crate root
// crate名与package 名相同

// src/lib.rs 如果还有这个文件
// 就是这个packag包含一个liabra crate
// library crate 的 crage root

// main.rs 与 lib.rs 都是入口文件 都是crate root
// cargo 会将 crate root文件交给rustc 来构建library 或 binary
// 可以同时含有 main.rs & lib.rs

// crate 的作用
// 可以将相关功能组合到一个作用域内，便于在项目间使用
// 防止冲突

// 定义 module 来控制作用域 和私有性
// 在一个 crate 内，将代码进行分组
// 增加可读性，易于复用
// 控制项目 （item）的私有性，public private

// 建立module
// mod 关键字
// 可嵌套
// 可包含 其他项 struct enum trait 常量
// 详见 👇 假设下方代码在 lib.rs中 （此时 还为创建lib.rs）

pub mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("eat")
        }

        pub fn seat_at_table() {}
    }

    mod serving {
        pub fn take_order() {}

        pub fn serve_order() {}

        pub fn take_payment() {}
    }
}

// path 在rust的模块中找到某个条目
// 两种形式
// 绝对路径：从crate root 开始，使用crate名或 字面值 crate
// 相对路径： 在当前模块使用 self（本身） , super(上一级)，或者 当前模块的标识符

pub fn eat_at_restaurant() {
    crate::package_crate_module::front_of_house::hosting::add_to_waitlist(); // 绝对路径  因为不是在 lib.rs 或在 main.rs中 所以需要加上文件名？？
    front_of_house::hosting::add_to_waitlist(); // 相对路径  👇=
    self::front_of_house::hosting::add_to_waitlist(); // self ☝ =
}

fn serve_order() {
    println!("order");
}

mod back_of_house {
    fn fix() {
        cook();
        super::serve_order(); // 调用上方的  相对路径
        crate::package_crate_module::serve_order() // 绝对路径;
    }
    fn cook() {
        println!("cook")
    }
}

// 大多数情况下 rust中 默认情况是 私有的
// 当 struct 中的字段为 pub 时，外部就可以访问修改它！！

// enum 中 如果 枚举为公共的 里面俄变体也变为公共的 需要注意

