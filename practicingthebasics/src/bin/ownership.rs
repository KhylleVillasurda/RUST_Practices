struct GroceryItem {
    id: i32,
    quantity: i32,
}

fn quantity(groc_qua: &GroceryItem) {
    println!("Grocery Quantity: {:?}", groc_qua.quantity);
}

fn grocery_id(groc_id: &GroceryItem) {
    println!("Grocery ID: {:?}", groc_id.id);
}
fn main() {
    let item1 = GroceryItem {
        id: 0010384823,
        quantity: 10,
    };

    let item2 = GroceryItem {
        id: 0010384824,
        quantity: 20,
    };

    quantity(&item1);
    grocery_id(&item1);

    quantity(&item2);
    grocery_id(&item2);
}
/*
Read the code above and I'll explain it to you

Firstly, we got struct GroceryItem which has two i32 fields, id and quantity.
Then we got two functions, which is quantity and grocery_id.

The quantity displays the quantity of the grocery item
and grocery_id displays the id of the grocery item.

The main goal here is to show that how you can borrow from the struct
without taking the ownership of it.

Then we got the main function the let item1 borrows the GroceryItem struct.

When a function or a variable takes ownership of a struct,
we will not able to access the struct through it directly.

We have a term called reference, which is a way to borrow the ownership
of a struct without taking the ownership of it.

When you add "&" before a variable, it means you are borrowing
the ownership of it, and you can access the struct through it.

Example here is the quantity and grocery_id functions.

As long as you borrow from the struct(using &) instead of owning it,
you can access the struct without a manipulating or duplicating the data.

a
 */
