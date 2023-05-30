fn main() {
    let mut msg = ConvolutedMessage {
        silent: true,
        count: 4,
        first_half: String::from("Hey "),
        second_half: String::from("there!"),
    };
    msg.try_ownership();
    msg.try_ownership();
}

// Print struct, debug mode

struct ConvolutedMessage {
    silent: bool,
    count: i16,
    first_half: String,
    second_half: String,
}

impl ConvolutedMessage {
    fn try_ownership(&mut self) {
        if !self.silent {
            for _ in 0..self.count {
                let mut message = self.first_half.clone();
                self.return_ownership(&mut message);
                ConvolutedMessage::immutable_borrow(&message);
                println!("Partially, {}", &message[0..3]);
            }
        } else {
            println!("Skipped");
            self.silent = false;
        }
    }

    fn return_ownership(&self, str: &mut String) -> () {
        str.push_str(&self.second_half.to_string());
    }

    fn immutable_borrow(str: &String) -> () {
        println!("{}", str);
    }
}
