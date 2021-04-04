use std::vec;

pub fn vec_learn() {
    let mut v: Vec<i32> = Vec::new(); // 空 vec
    v.push(1);

    println!("{:#?}", v);

    let v = vec![1,2,3];
    println!("{:#?}", v);
    
}
