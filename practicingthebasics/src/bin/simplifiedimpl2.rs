enum Color {
    Black,
    White,
}

impl Color {
    fn print(&self) {
        match self {
            Color::Black => println!("Black"),
            Color::White => println!("White"),
        }
    }
}

struct Dimensions {
    height: f64,
    width: f64,
    depth: f64,
}

impl Dimensions {
    fn print(&self) {
        println!("Box Height: {}", self.height);
        println!("Box Width: {}", self.width);
        println!("Box Depth: {}", self.depth);
    }
}

struct ShippingBox {
    color: Color,
    dimensions: Dimensions,
    weight: f64,
}

impl ShippingBox {
    fn new(weight: f64, color: Color, dimensions: Dimensions) -> Self {
        Self {
            weight,
            color,
            dimensions,
        }
    }
    fn print(&self) {
        self.color.print();
        self.dimensions.print();
        println!("Weight: {:?}", self.weight)
    }
}

fn main() {
    let small_dimensions = Dimensions {
        width: 1.0,
        height: 2.0,
        depth: 3.0,
    };

    let black_dimension = Dimensions {
        width: 2.0,
        height: 4.0,
        depth: 6.0,
    };

    let small_box = ShippingBox::new(5.0, Color::White, small_dimensions);
    let black_dimension = ShippingBox::new(10.0, Color::Black, black_dimension);
    small_box.print();
    black_dimension.print();
}
