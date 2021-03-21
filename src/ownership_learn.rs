use std::usize;

//所有权规则：
//1：每个值都有一个变量，这个变量是该值的所有者
//2：每个值同时只能有一个所有者
//3：当所有者超出作用域（scope）时候，该值将被删除

pub fn ownership() {
    string_example();

    owner_move_copy();

    action_not_take();

    action_not_take2();

    limit_of_ref();
}

fn string_example() {
    let mut s = String::from("hello");
    // s = s + ",world";
    s.push_str(",world");

    println!("{}", s);
} //drop(s);

fn scope() {
    //x不可用
    let _x = 1; //x可用
                // 可以对 x 进行相关操作
} //x 作用域到此结束，x不再可用 销毁

//变量与数据交互的方式： 移动（Move）
// 多个变量可以与同一个数据使用一种独特的方式来交互
// 移动！！！  s1 move to s2;只有 s2 有效，所以离开作用域，只有 s2 被drop
//rust 不会自动创建数据的深拷贝
fn move_method() {
    let s1 = String::from("hello");
    let s2 = s1; //赋值完 s2 后，s1将失效  仅仅拷贝了 stack

    // println!("{}",s1) ;//发生了 借用 borrow 编译报错

    // 如果想要深拷贝 使用 clone() 拷贝 stack 与 heap
    // let s2 = s1.clone();
    // println!("{}{}",s1,s2);
}
/*  stack👇： 一个指向存放字符串内容的内存的指针
            一个长度；一个容量； 以上这些东西存放在 stack 上
            存放字符串内容的部分在 heap 上
            长度 len，就是存放字符串内容所需的字节数
            容量 capacity 是指 string从操作系统总共获得内容的总字节数
            note:一个英文字母占用 1个字节
             s1
    name      |  value  |      index        value
    ptr       |     ----|----->   0    |       h
    len       |    5    |         1    |       e
    capacity  |    5    |         2    |       l
                                  3    |       l
                                  4    |       0

*/

/*
整数等类型,在编译时间具有已知大小的完全存储在堆栈上，因此实际值的副本可以快速制作。
Stack 上的数据复制：
copy trait ，可以用于像整数这样完全存放在stack上面的类型
如果一个类型实现了 Copy 这个 trait，那么旧的变量在赋值后仍然可用
如果一个类型或者该类型的一部分实现了 Drop trait ，那么Rust不允许让他再去实现copy trait; trait:特点 特征

*/
fn stack_only_copy() {
    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y);
}

/*
一些拥有copy trait 的类型
任何简单标量的组合类型都可以是copy的
任何需要分配内存或者某种资源的都不是copy的

copy trait 👇
所有的整数类型，例如 u32
bool
char
所有的浮点类型 f32 f64
tuple,仅当其所有的字段都是copy的 ，如 (i32,i32)：是 ；(i32,String) 否；

*/

// 所有权与函数
// 语义上，将值传递给函数和把值赋给变量是类似的
// 将值传递给函数将发生 移动(Move) 或 复制（copy）

fn owner_move_copy() {
    let s = String::from("hello"); // s comes into scope

    takes_ownership(s); // s's value moves into the function...
                        // ... and so is no longer valid here

    let x = 5; // x comes into scope

    makes_copy(x); // x would move into the function,
                   // but i32 is Copy, so it’s okay to still use x afterward
} // Here, x goes out of scope, then s. But because s's value was moved, nothing  special happens.

fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) {
    // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of sc

// 返回值与作用域
// 函数在返回值的过程中 同样会发生所有权转移
// 一个变量的所有权总是遵循同样的模式：
// 把一个值赋值给其他变量时就会发生移动（i32类的为copy）
// 当一个包含heap数据的变量离开作用域时，它的值就会被 drop 函数清理，除非数据的所有权转移到另一个变量上了

fn take_give() {
    let s1 = gives_ownership(); // gives_ownership moves its return
                                // value into s1

    let s2 = String::from("hello"); // s2 comes into scope

    let s3 = takes_and_gives_back(s2); // s2 is moved into
                                       // takes_and_gives_back, which also
                                       // moves its return value into s3
} // Here, s3 goes out of scope and is dropped. s2 goes out of scope but was
  // moved, so nothing happens. s1 goes out of scope and is dropped.
fn gives_ownership() -> String {
    // gives_ownership will move its
    // return value into the function
    // that calls it

    let some_string = String::from("hello"); // some_string comes into scope

    some_string // some_string is returned and
                // moves out to the calling
                // function
}

// takes_and_gives_back will take a String and return one
fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes into
    // scope

    a_string // a_string is returned and moves out to the calling function
}

// 让函数使用某个值，但不获得其所有权
//
fn action_not_take() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1); //s1 move to s

    println!("The length of '{}' is {}.", s2, len); //s2 还是 s的所有权
}

fn calculate_length(s: String) -> (String, usize) {
    let len = s.len();
    (s, len)
}

// 引用与借用（把引用作为函数参数这个行为就叫做借用）
// & 符号表示引用：允许你引用某写值而不取得其所有权
// 将变量 方法名 入参 前加上 mut 即可修改 借用的
// 可变引用限制： 有特定作用域内，对某一块数据，只能有一个可变的引用

fn action_not_take2() {
    let mut s1 = String::from("hello");

    let len = calculate_length2(&mut s1); //& 表示引用

    println!("The length of '{}' is {}.", s1, len);
}
fn calculate_length2(s: &mut String) -> usize {
    //&String  s(指针) 指向 s1，即 s 为 s1的引用
    s.push_str(" world");
    s.len()
    // s 指向的 是s1 不会被清理，并没有s1的所有权
}

fn limit_of_ref() {
    println!("/n----- limit of ref---");
    let mut s = String::from("hello");
    let s1 = &mut s;
    // let s2 = &mut s;
    /*
    cannot borrow `s` as mutable more than once at a time
     let s1 = &mut s;
        |              ------ first mutable borrow occurs here
     let s2 = &mut s;
        |              ^^^^^^ second mutable borrow occurs here
    */
    //  不可同时借用 可变引用超过 1 次；
    // 好处，可在编译时防止数据竞争
    println!("the length of '{}' is {}", s1, s1);
}
/*
三种行为会发生数据竞争👇
1：两个或多个指针，同时访问用一个数据
2：至少有一个指针用于写入数据
3：没有使用任何机制来哦同步对数据的访问
*/

//不可以同时拥有可变引用与不可变引用
//可以同时拥有多个不可变引用
// 不可以同时拥有超过 1 个可变引用

fn both_mut_imut() {
    let mut s = String::from("hello");
    let r1 = &s;
    let r2 = &s;
    // let r3 = &mut s;
    //cannot borrow `s` as mutable because it is also borrowed as immutable
    println!("{}{}{}", r1, r2, "r3");
}

//悬空指针， dangling references
// 一个指针引用了内存的某个地址，而这块内存可能已经释放并分配给其他人引用了
// rust中，编译器可以保证引用永远不是 dangling references

// fn dangling_ref1() {
//     let s = dangling_ref2();
// }
// fn dangling_ref2() -> &String {
//                     ^ expected named lifetime parameter
//     let s = "s";
//     s
// }
/*
引用规则：只能满足一下条件之一
1：一个可变引用
2：任意数量不可变引用
*/