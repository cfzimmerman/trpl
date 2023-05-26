use dialoguer::{console::Term, theme::ColorfulTheme, Select};
use std::io;
use std::io::Write;

fn f_to_c(f: f32) -> f32 {
    (f - 32.) * (5. / 9.)
}

fn c_to_f(c: f32) -> f32 {
    ((9. / 5.) * c) + 32.
}

fn main() -> std::io::Result<()> {
    let modes = vec!["F to C", "C to F"];
    let selection = Select::with_theme(&ColorfulTheme::default())
        .items(&modes)
        .default(0)
        .interact_on_opt(&Term::stderr())?;

    let index = selection.unwrap_or_default() as i32;

    loop {
        print!("Temperature to convert: ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        let input: f32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let conversion = match index {
            0 => f_to_c(input),
            1 => c_to_f(input),
            _ => panic!("received undefined index {index}"),
        };

        println!("{conversion}");

        break;
    }

    Ok(())
}
