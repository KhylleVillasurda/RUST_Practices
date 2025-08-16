struct Drink {
    name: String,
    flavour: Flavour,
    ounces: i32,
}

enum Flavour {
    Chocolate,
    Vanilla,
    Strawberry,
}

fn main() {
    let my_drink = Drink {
        name: String::from("Chocolate Milkshake"),
        flavour: Flavour::Chocolate,
        ounces: 8,
    };
    println!("Drink Name: {}", my_drink.name);
    match my_drink.flavour {
        Flavour::Chocolate => println!("Chocolate Flavour"),
        Flavour::Vanilla => println!("Vanilla"),
        Flavour::Strawberry => println!("Strawberry Flavour"),
    }
    println!("Ounces: {}", my_drink.ounces);
}

/* Reference snippet for struct definition
   This is a simple example of a struct in Rust.
struct ShippingBox {
    length: i32, //Remember that you need to define the type of each field
    width: i32,
    height: i32,
}

    let my_box = ShippingBox { //It's like using the template from ShippingBox
        length: 10,
        width: 5,
        height: 2,
    };

    let box = my_box.height; //you can access the heigh field like this
    println!("The height of the box is: {:?}", box);

*/
