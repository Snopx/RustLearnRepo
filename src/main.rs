fn main() {
    let mut a: i32 = 32;
    println!("before hide a ={}", a);

    a = 3;
    println!("overwritten a={}", a);

    let arr: [i32; 3] = [1, 2, 3];
    for elem in arr.iter() {
        println!("{}", elem);
    }
    print_arr(arr);
}

fn print_arr(arr: [i32; 3]) {
    println!("----------");
    for elem in &arr {
        println!("{}", elem);
    }
}
