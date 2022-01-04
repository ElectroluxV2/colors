use crate::Color::{Orange, Red};

fn main() {
    println!("Normal {} Text {}", Red.write("Red text"), Orange.write("Orange text"));
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
