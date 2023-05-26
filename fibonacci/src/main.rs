use std::io::{self, Write};

fn fib(num: i32) -> i32 {
    if num <= 1 {
        return num;
    }
    return fib(num - 1) + fib(num - 2);
}

fn main() {
    loop {
        print!("Fibonnaci number: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("read input failed");

        let num: i32 = match input.trim().parse() {
            Ok(n) => n,
            Err(_) => continue,
        };

        if num < 0 {
            continue;
        }

        let res = fib(num);

        println!("{res}");

        break;
    }
}
