use std::io;
use rand::Rng;
use std::cmp::Ordering;
use std::io::Read;

fn main() {
    println!("Guess the number!");
    let random_num = rand::thread_rng().gen_range(1..=100);
    let mut amount_tries = 0;
    let mut won = 0;
    let mut amount_of_tries_max = String::new();
    println!("How many tries do you want?");
    io::stdin().read_line(&mut amount_of_tries_max).expect("Failed to read line");
    let amount_of_tries_max = amount_of_tries_max.trim().parse::<i32>().unwrap();

    while amount_of_tries_max > amount_tries {
    println!("Please input your guess.");
    //println!("{}",{random_num});
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Only type a number!");
                continue;
            },
    };
    println!("You guessed: {guess}");

    match guess.cmp(&random_num) {
        Ordering::Less => println!("Too low"),
        Ordering::Greater => println!("Too high"),
        Ordering::Equal => {
            println!("You Win!");
            won = 1;
            break;
        }
    }
        amount_tries += 1
    }
    if won == 0 {
        println!("You Used all of your tries! Goodbye.");
    }
}
