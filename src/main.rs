//Ordering by mod refer sequence like below
// mod guessing_game; //pre
// mod variables_learn; //1
// mod learn_type; //2
// mod control_flow;//3
// mod ownership_learn;//4
mod slice_learn;
fn main() {
    // guessing_game::guessing_game();
    // variables_learn::variables_learn();
    // learn_type::learn_type();

    // let int = int_method();
    // println!("{}",int);

    // control_flow::control_flow();
    // ownership_learn::ownership();
    slice_learn::slice_learn();
    slice_learn::slice_come();
    slice_learn::slice_learn_adv();
}

//函数的声明 返回类型 ->type
fn int_method() -> i32 {
    3 // 两种返回形式，直接写返回值 或者 return value;
      //return 3;
}
