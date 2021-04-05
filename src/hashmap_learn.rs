use std::collections::HashMap;
pub fn hashmap_learn() {
    let mut h: HashMap<String, i32> = HashMap::new();
    h.insert("k".to_string(), 1);

    for (k, v) in &h {
        println!("{}:{}", k, v)
    }
    // 数据存储在 heap上
    // 同构的；即 ， 所有的数据必须遵循 k v 所定义的类型

    let teams = vec!["black".to_string(), "white".to_string()];
    let ini = vec![10, 15];

    let scores: HashMap<_, _> = teams.iter().zip(ini.iter()).collect();

    for (k, v) in &scores {
        println!("{}:{}", k, v);
    }

    // 所有权
    // 对于实现了 Copy trait 如（i32）,值会被复制到 hashmap中
    // 对于拥有所有权的值 如 (string),值会被移动,所有权会转移到 hashmap中
    // 在hashp 有效期间，被引用的值必须保持有效
    let mut h = HashMap::new();
    let s = "s".to_string();
    h.insert(&s, 3);
    //drop(s);
    println!("{:#?}", h);

    // 获取 hashmap中的值

    let x = h.get_key_value(&s);
    match x {
        Some((k, v)) => println!("{}:{}", k, v),
        _ => println!("none"),
    };
    // HashMap 大小可变 ， 每个K 只能对应一个 value
    // 更新hashmap中的数据
    // K已经存在： 替换现有V， 保留现有V ， 合并现有V与新的V
    // K不存在： 添加一个新的 K v

    // 替换
    h.insert(&s, 5);
    let v = h[&s];
    println!("after update {}:{}", &s, v);

    println!("-----------");
    // 保留与插入新KV
    let a = "a".to_string();
    h.entry(&a).or_insert(50); // new insert
    h.entry(&s).or_insert(50); // remain

    for (k, v) in &h {
        println!("{}:{}", k, v);
    }

    // 基于现有值更新V
    //
    let txt = "hellow world excell world";
    let mut map = HashMap::new();
    for word in txt.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1; //解引用
    }
    println!("{:#?}", map);

    // 默认情况下 ，hashmap 使用加密功能强大的 Hash函数，可以抵抗拒绝服务(Dos)攻击
    // 不是可用的最快Hash算法
    // 但具有更好的安全性
    // 可以指定不同的hasher 来切换到另一个函数
    // hasher 是实现 BuildHasher trait 的类型
}
