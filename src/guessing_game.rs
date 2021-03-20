use rand::Rng;
use std::cmp::Ordering;
use std::io;

pub fn guessing_game() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..100);
    let mut time = 0;
    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess: u32 = guess.trim().parse().expect("You need type a num");

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("you win");
                break;
            }
        }
        time += 1;
    }
    println!("you guess {} times",time);
}
