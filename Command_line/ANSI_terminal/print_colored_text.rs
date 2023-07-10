// ANSI escape code로 console의 글자 색이나 스타일을 변경
use ansi_term::Colour;

fn main() {
    println!("This is {} in color, {} in color and {} in color",
            // Colour는 색상을 나타내는 열거형
            Colour::Red.paint("red"),
            Colour::Blue.paint("blue"),
            Colour::Green.paint("green"));
}