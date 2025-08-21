enum Ingot {
    Iron,
    Gold,
}

fn ingot_display(i: Ingot) {
    match i {
        Ingot::Iron => println!("This is an iron ingot"),
        Ingot::Gold => println!("This is an gold ingot"),
    };
}

fn main() {
    let sword = Ingot::Iron;
    let sword2 = Ingot::Gold;
    ingot_display(sword);
    ingot_display(sword2);

    /*
    You can never duplicate a mutable reference in Rust.
    */
}
