pub fn newstring() {
    let s = String::from("init a string");

    let mut s = "init a string ".to_string();

    s.push('y');
    s.push_str(" string");
    let s3 = s + " 123"; // 此时 s 的所有权 move 到的 s3;
    println!("{}", s3);

    // + 连接字符串 实际上使用类似
    // fn add(self,s:&str)->String 这个方法
    // deref coercion 解引用强制转换
    // add 方法就是使用了 deref coercion
    // String == Vec<u8>

    let f = format!("{}-{}-{}", "1", "2", "3");
    println!("{}", f);

    // Sting 无法使用 index  &str[] 字符串切片可以 &s[..3]

    for b in f.bytes() { //字节
        println!("b = {}", b);
    }
    let f = "嗳哟".to_string();
    for c in f.chars() {// 字符 标量值
        println!("c = {}", c);
    }

    // rust 不允许string使用索引，last reason :
    // 索引操作应该为 O(1) 耗时
    // 而String无法保证需要遍历所有内容，来确定you多少个合法字符
}
