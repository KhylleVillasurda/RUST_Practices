fn main() {
    /*
    Take a look at thie following code.

    Although we can can never duplicate a mutable, but if you scope the r1
    out of the r2 becomes valid but why?

    Think of it like r2  and r1 cannot see each other, therefore they can
    both access the ssame data without any issues. Scopes ensure that isolates itself
    from the other mutable references.
    */
    let mut s = String::from("hello");
    {
        let r1 = &mut s;
        println!("{}", r1);
    }
    let r2 = &mut s;

    println!("{}", r2);
}
