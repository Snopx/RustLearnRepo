pub fn slice_learn() {
    let mut s = String::from("hello world");

    let world_index = first_world(&s); //返回了索引位置
                                       // 此时 world_index 一旦 脱离了上下文，就无法保证它的有效性 （一直会是固定的）
    s.clear(); //即使 清空了字符串 world_index 与 字符串 's' 没有关联
               //为了确保 同步性，往往需要多大量工作
               //字符串切片 能解决
    println!("index={}", world_index);
}

fn first_world(s: &String) -> usize {
    let bytes = s.as_bytes();
    // iter 迭代器 ：依次返回集合中的每个元素
    // enumerate ： 会将Iter<_,T> 的每个元素包装 成 (i,val)元组 i:index val:value
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            // b' ' 字符 空格的字面值
            return i; // 返回索引
        }
    }
    s.len()
}

//note： 字符串切片的范围索引必须发生在有效的 UTF-8字符边界内
// 如果尝试从一个多字节的字符中创建字符串切片，程序会报错退出
// 中文占用 3个字节 因为是 UTF-8 ；UTF-16 4个字节
pub fn slice_come() {
    let s = String::from("你好 world");
    // let hello = &s[0..5];// [) 前等后不等
    // let world = &s[6..11];

    // 语法糖
    let hello = &s[..6]; // [) 前等后不等
    let world = &s[7..];

    let whole = &s[..];
    println!("{},{};{}", hello, world, whole);
}

//加强吧
pub fn slice_learn_adv() {
    let mut s = String::from("hello world");

    let world_index = first_world_adv(&s);
    //           ------ first mutable borrow occurs
    // s.clear();
    // ^ second mutable borrow occurs here
    println!("index={}", world_index);
}

fn first_world_adv(s: &String) -> &str {
    // &str 字符串切片
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            // b' ' 字符 空格的字面值
            return &s[..i]; // 返回索引
        }
    }
    &s[..]
}

// 将字符串 切片作为参数
// 此时 参数 s 可以传入 &str 切片类型 也可以传入 String类型参数
fn str_as_param(s: &str) -> &str {
    s
}
