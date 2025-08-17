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
//Method 1
fn display_drink() {
    //you can display  a drink with struct and enums into a function
    let d = Drink {
        name: String::from("Tequila Sunrise"),
        flavour: Flavour::Vanilla,
        ounces: 12,
    };

    println!("Drink Name: {}", d.name);
    match d.flavour {
        Flavour::Chocolate => println!("Chocolate Flavour"),
        Flavour::Vanilla => println!("Vanilla"),
        Flavour::Strawberry => println!("Strawberry Flavour"),
    }
    println!("Ounces: {}\n", d.ounces);
}

fn main() {
    //Method 2, you can just directly define the values here with your struct
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
    println!("Ounces: {}\n", my_drink.ounces);
    display_drink();
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
