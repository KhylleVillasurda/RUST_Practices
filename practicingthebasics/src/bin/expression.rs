fn print_message(c: bool) {
    match c {
        true => println!("its big"),
        false => println!("its small"),
    }
}

fn main() {
    let x = 150;
    let verify = x > 100;
    println!("{:?}", verify);

    print_message(verify);
}

//Print "its big" if a variable is > 100 and Print "its small" if a variable is <= 100
//Use a boolean variable set to an expression that determines whether the value is >100 or <=100
//Use a function to print the messages
//Use a match expression to determine which message to print
