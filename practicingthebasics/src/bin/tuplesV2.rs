fn cartesian() -> (i32, i32) {
    (-10, 5)
}

fn main() {
    let (x, y) = cartesian();
    if y > 5 {
        println!("y is greater than 5");
    } else if y < 5 {
        println!("y is less than 5");
    } else if y == 5 {
        println!("y is equal to 5");
    } else {
        println!("Invalid Input")
    }
    //test here
    let num = 3;
    let spare = num < 5;
    println!("{}", spare);
}
