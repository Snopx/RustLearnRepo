use std::vec;

pub fn vec_learn() {
    let mut v: Vec<i32> = Vec::new(); // 空 vec
    v.push(1);

    println!("{:#?}", v);

    let v = vec![1, 2, 3];
    println!("{:#?}", v);

    // index out of bounds

    let v = vec![1, 2, 3, 4, 5];
    let third = v[3];
    println!("{}", third);

    match v.get(100) {
        Some(onehundred) => println!("{}", onehundred),
        None => println!("panic:: the length of v index you set is {}", v.len()),
    }

    // 可变引用与不可变引用 ！！ 报错 ,同时存在的可变引用有且只能有一个
    let mut v: Vec<i32> = vec![1, 2, 4];
    let first = &mut v[0];
    // v.push(5);//pub fn push(&mut self, value: T)
    println!("first is {}", first);

    //vec 在内存中是连续的，在该vec添加元素时候，内存中可能没有这么大的内存块，会重新分配内存块，此时原有引用就失效了，↑；

    //遍历
    for i in v.iter(){
        println!("{}",i);
    }
    // 修改
    for i in &mut v {
        *i +=10;// *解引用
        println!("{}",i);
    }

    //rust 编译时需要知道 vec 的类型，这样它就知道在heap上，需要多少内存
    let row = vec![
        SheetCell::Int(1),
        SheetCell::Text("blue".to_string()),
        SheetCell::Float(23.23)
    ];

    for r in row.iter() {
        match r {
            SheetCell::Int(i)=>println!("{}",i),
            SheetCell::Float(f)=>println!("{}",f),
            SheetCell::Text(t)=>println!("{}",t),
        }
    }
    println!("{:#?}",row);

}
#[derive(Debug)]
enum SheetCell{
    Int(i32),
    Float(f64),
    Text(String)
}
