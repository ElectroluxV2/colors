use crate::Color::{Orange, Red};

fn main() {
    println!("Normal {} Text {}", Red.write("Red text"), Orange.write("Orange text"));

    println!("{} days", 3/* TODO */);

    println!("{0}, this is {1}. {1}, this is {0}.", "Alice", "Bob");

    println!("Today's agenda is: {}, {0} and {subject}.", subject = "Math");

    println!("{:b}", 2137);

    println!("{number:0>width$}", number = 1, width = 6);

    println!("{:05.2}€", 5.2);
}

#[derive(Debug)]
enum Color {
    Red,
    Orange
    // ...
}

impl Color {
    fn value(&self) -> (u8, u8, u8) {
        match *self {
            Color::Red => (255, 65, 54),
            Color::Orange => (255, 133, 27)
            // ...
        }
    }

    fn write(&self, text: &str) -> String {
        format!("\x1b[38;2;{};{};{}m{}\x1b[0m", self.value().0, self.value().1, self.value().2, text)
    }
}
