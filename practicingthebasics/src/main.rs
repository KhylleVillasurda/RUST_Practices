fn main() {
    let x = 10;
    let y = 100;
    let z = 10000;

    if x > y {
        // here check if x is greater than y then if it's true it will continue to the next condition
        if x > z {
            //afterwards if x is greater than y it proceeds to check if x is also greater than z
            println!("x is bigger than y and z");
        } else {
            println!("x is smaller than z but bigger than y");
        }
    } else {
        print!("x is smaller than y");
    }
}
