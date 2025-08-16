fn main() {
    let mut i = 6;
    while i >= 1 {
        //the condition here says "if i is greater than 1 it will proceed to loop"
        println!("{:?}", i);
        i = i - 1; // here is where the i gets decremented by 1 till the condition is false
    }
    println!("Looping Done"); // this will print after the loop is done
}
