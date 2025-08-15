fn main() {
    display_results();
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn display_results() {
    println!("The result of a and b is: {:?}", add(1, 2));
}

/*
Better Answer
fn main() {
    let result = add(1, 2);
    display_results();
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn display_results(result: i32) {
    println!("The result of a and b is: {:?}", result);
}
*/
