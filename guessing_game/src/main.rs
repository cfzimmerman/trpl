use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {

    let min: i8 = 1;
    let max: i8 = 100;

    println!("Guess the number!");

    let secret_num = rand::thread_rng().gen_range(min..=max);

    loop {

        println!("\nPlease take a guess between {min} and {max}:");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read guess");

        let guess: i8 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_num) {
            Ordering::Less => println!("Too low"),
            Ordering::Greater => println!("Too high"),
            Ordering::Equal => { 
                println!("🎉 Correct!");
                break;
            },
        }
    }
}
