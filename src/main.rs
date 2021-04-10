use learn_trait::learn_trait::Summary;

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
<<<<<<< HEAD
mod generic_learn;
=======
>>>>>>> 29828d8f0ed00d5bfa3ea664c2176b1a13a13d35
mod hashmap_learn;
mod package_crate_module;
mod string_learn;
mod use_learn;
mod vec_learn;
<<<<<<< HEAD

mod learn_trait;
=======
>>>>>>> 29828d8f0ed00d5bfa3ea664c2176b1a13a13d35
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
    // use crate::generic_learn::generic_learn;
    // let number_list = vec![30, 20, 3, 342, 24];

    // let v = generic_learn::sort_generic(&number_list);
    // println!("the largest num is {}", v);

    // let first = number_list[0];
    // println!("first num is {}", first);
    // println!("and the number list is {:#?}", number_list);

    // let char_list = vec!['y', 'm', 'z', 'q'];
    // let v = generic_learn::sort_generic(&char_list);
    // println!("the largest char is {}", v);
    // println!("and the char list is {:#?}", char_list);

    // let integer = generic_learn::Point::new_point(3, 5);

    // println!("x:{},y:{}", integer.get_x(), integer.get_y());
    // println!("{:#?}", integer);

    // let charater = generic_learn::Point::new_point("x", "y");

    // println!("x:{},y:{}", charater.get_x(), charater.get_y());
    // println!("{:#?}", charater);
    guessing();

    use crate::learn_trait::*;
    let tweet = learn_trait::Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };



    println!("1 new tweet: {}", tweet.summarize());
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
