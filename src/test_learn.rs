#[test] // 将函数变为测试函数 使用 cargo test
fn test_learn() {
    println!("test learn");
    let t = 1 == 1;
    assert!(t);
}
