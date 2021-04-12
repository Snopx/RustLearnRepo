pub fn add_two(a: i32) -> i32 {
    a + 2
}
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn add_two_and_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn add_three_and_two() {
        assert_eq!(5, add_two(3));
    }

    #[test]
    fn one_hundred() {
        assert_eq!(102, add_two(100));
    }
    #[test] // 将函数变为测试函数 使用 cargo test
    fn test_learn() {
        println!("test learn");
        let t = 1 == 1;
        assert!(t);
    }

    #[test]
    fn madd() {
        assert!(true);
    }

    #[test]
    #[ignore]
    fn expensive_test() {
        // code that takes an hour to run
    }

    // cargo test 会将所有标记为 #[test] 的方法测试
    // cargo test test_learn  表示只对 test_learn() 进行测试
    // cargo test add  表示测试包含 add 字符的 方法进行测试

    // #[ignored] 表示使用cargo test时，忽略该测试
    // cargo test -- --ignored 表示 测试忽略的方法  更多用法 cargo test -- --help 查看

    //  #[should_panic] attribute after the #[test] attribute and before the test function it applies to.  表示该方法会panic ，可以在该标注中加入参数（字符串），表示会触发的报错提示
    // like  #[should_panic(expected = "Guess value must be less than or equal to 100")]

    // #[cfg(test)] 使用该标注的 module 进行cargo test时候只会编译该部分
    // The #[cfg(test)] annotation on the tests module tells Rust to compile and run the test code only when you run cargo test, not when you run cargo build. 


    // cargo test --test integration_test
    // This command runs only the tests in the tests/integration_test.rs file. tests文件夹下的 integration_test.rs 文件
}
