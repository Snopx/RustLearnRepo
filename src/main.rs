mod guessing_game;
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
mod generic_learn;
mod hashmap_learn;
mod package_crate_module;
mod string_learn;
mod use_learn;
mod vec_learn;

mod learn_trait;
use learn_trait::learn_trait::{notify, Summary};

mod algorithms;

mod life_time_learn;

mod test_learn;

mod io_learn;
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

    // let string_list = vec!["word".to_string(), "we".to_string()];
    // let v = generic_learn::sort_generic(&string_list);
    // println!("the largest string is {}", v);
    // println!("and the string list is {:#?}", string_list);

    // let integer = generic_learn::Point::new_point(3, 5);

    // println!("x:{},y:{}", integer.get_x(), integer.get_y());
    // println!("{:#?}", integer);

    // let charater = generic_learn::Point::new_point("x", "y");

    // println!("x:{},y:{}", charater.get_x(), charater.get_y());
    // println!("{:#?}", charater);
    // guessing_game::guessing_game::guessing();

    // use crate::learn_trait::*;
    // let tweet = learn_trait::Tweet {
    //     username: String::from("horse_ebooks"),
    //     content: String::from("of course, as you probably already know, people"),
    //     reply: false,
    //     retweet: false,
    // };

    // println!("1 new tweet: {}", tweet.summarize());

    // notify(&tweet);

    // println!("tweet's username: {}", tweet.username);

    // let mut number_list = vec![30, 20, 3, 342, 24];
    // println!("before sort :{:#?}", number_list);
    // algorithms::bubble_sort(&mut number_list);
    // println!("after sorted :{:#?}", number_list);

    // life_time_learn::life_time_learn();
    /*  IO-LEARN

    use std::env;
    use std::process;
    let args: Vec<String> = env::args().collect(); // 获取环境参数
    let config = io_learn::Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1); //process didn't exit successfully: `target\debug\learn_rust.exe` (exit code: 1)
    });
    if let Err(e) = io_learn::run(config) {
        eprintln!("Application error: {}", e);

        process::exit(1);
    }
    // 使用 cargo run to poem.txt > output.txt  将结果写入到 output.txt中
    */
}
