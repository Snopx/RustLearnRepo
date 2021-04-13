use std::fmt::Display;
pub fn life_time_learn() {
    let s1 = String::from("abcd");
    let s2 = "xyz";

    let result = longest(s1.as_str(), s2);
    println!("longest one is {}", result);
}

// rust 中的 借用检查器
fn ref_check() {
    // let r;           a
    // {                b
    //     let x = 5;
    //     r = &x;
    // }
    // println!("{}", r);

    // a的生命周期大于 b的生命周期
}
// note ，这个生命周期 ’a 是取的 x,y重叠的部分，也就是比较短的那个生命周期
// 指定生命周期参数的方式依赖于 函数所做的事情
// 从函数返回引用时，返回类型额生命周期参数需要与其中一个参数的生命周期匹配
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

//note: 生命周期的标注不做改变引用的生命周期长度
// 当指定了泛型生命周期参数，函数可以接受带有任何生命周期的引用
// 生命周期的标注:描述了多个引用的生命周期间的关系，但不影响生命周期

//生命周期参数名：
// 以 '开头 通常全小写且很短，大部分人使用'a

// 如果返回的引用没有指向任何参数，那么它只能引用函数内创建的值
// 此时会发生 悬挂指针，

// 每个引用都有生命周期
// 还有一些情况是 生命周期省略规则

// 输入/出 生命周期
// 函数/方法的参数：输入生命周期
// 函数/方法的返回值：输出生命周期

// 生命周期省略规则
// 1 应用于输入生命周期
// 2、3 应用与输出生命周期
// 如果编译器应用完3个规则后，仍然有不确定的生命周期引用->报错
// 这些规则适用于 fn 与 impl 块

// 规则1 ： 每个引用类型的参数都有自己的生命周期
// 规则2 ： 如果只有1个输入生命周期参数，那么该生命周期被赋给所有的输出生命周期
// 规则3 ： 如果有多个输入生命周期参数，其中一个是 &self 或者 &mut self,那么self 的生命周期会被赋给所有的输出生命周期参数

//  fn first_word(s: &str) -> &str { 应用规则1 后👇
//  fn first_word<'a>(s: &'a str) -> &str {  应用规则2 后👇
//  fn first_word<'a>(s: &'a str) -> &'a str {

//  fn longest(x: &str, y: &str) -> &str { 应用规则1 后👇
//  fn longest<'a, 'b>(x: &'a str, y: &'b str) -> &str {
//  此时 还是未能判断返回值的生命周期 ——>报错

struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

// 静态生命周期 'static
// 例如：👇 所有的字符串字面值都拥有 'static 生命周期，存储在二进制程序中，总是可用的
// let s: &'static str = "I have a static lifetime.";
// 为引用指定 'static 生命周期前要注意，是否要引用在程序整个生命周期内都存活
//

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
