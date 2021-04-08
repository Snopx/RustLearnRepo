//Ordering by mod refer sequence like below
// mod variables_learn; //1
// mod learn_type; //2
// mod control_flow;//3
// mod ownership_learn;//4.1
// mod slice_learn;//4.2
// mod struct_learn;//5.1
// mod struct_demo;//5.2
//mod enum_learn;//6

// mod match_learn;//6.1
mod hashmap_learn;
mod package_crate_module;
mod string_learn;
mod use_learn;
mod vec_learn;
fn main() {
    // variables_learn::variables_learn();
    // learn_type::learn_type();

    // let int = int_method();
    // println!("{}",int);

    // control_flow::control_flow();
    // ownership_learn::ownership();
    // slice_learn::slice_learn();
    // slice_learn::slice_come();
    // slice_learn::slice_learn_adv();

    // struct_learn::struct_learn();

    // struct_demo::struct_demo();

    // enum_learn::enum_learn();

    // match_learn::match_learn();

    //
    // crate::package_crate_module::front_of_house::hosting::add_to_waitlist(); // 绝对路径
    // use crate::use_learn::front_of_house_a::hosting;
    // hosting::add_to_waitlist();

    // vec_learn::vec_learn();

    // string_learn::newstring();

    // hashmap_learn::hashmap_learn();

    guessing();
}

//函数的声明 返回类型 ->type
fn int_method() -> i32 {
    3 // 两种返回形式，直接写返回值 或者 return value;
      //return 3;
}

use rand::Rng;
use std::cmp::Ordering;
use std::io;
mod guessing_game; //pre
use crate::guessing_game::guessing_game::Guess;
pub fn guessing() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..100);
    let mut time = 0;
    loop {

        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        time += 1;
        let guess: Guess = match guess.trim().parse() {
            Ok(num) => Guess::new(num),
            Err(_) => {
                println!("please type a num!");
                continue;
            }
        };

        println!("You guessed: {}", guess.value());

        match guess.value().cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("you win 🎇🎇🎇🎇🎇");
                break;
            }
        }
    }
    println!("you guess {} times", time);
}
