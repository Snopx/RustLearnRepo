const MAX_VALUE: u32 = 10_000;//常量
pub fn variables_learn() {
    println!("variables learn");

    let x = 5;

    println!("the imutable value of x is {}", x);

    //shadowing the variable "x" above 隐藏性:使用新声明的的变量 x
    let mut x = 10;
    println!("the before modified value of x is {}", x);

    x = 20;
    println!("the after modified value of x is{}", x);

    println!("const value of MAX_VALUE IS {}", MAX_VALUE);
}
