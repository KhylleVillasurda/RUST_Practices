enum Color {
    Red,
    Green,
    Blue,
}

fn display_color(c: Color) {
    match c {
        Color::Red => println!("The color is Red!"),
        Color::Green => println!("The color is Green!"),
        Color::Blue => println!("The color is Blue!"),
    }
}

fn main() {
    display_color(Color::Red);
}
