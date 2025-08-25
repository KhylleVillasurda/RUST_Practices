struct Dimensions {
    width: f64,
    height: f64,
}

enum Color {
    Black,
    White,
}

struct ShippingBox {
    dimensions: Dimensions,
    color: Color,
}

impl ShippingBox {
    fn boxv1() -> Self {
        Self {
            dimensions: Dimensions {
                width: 10.5,
                height: 10.5,
            },
            color: Color::White,
        }
    }

    fn boxv2() -> Self {
        Self {
            dimensions: Dimensions {
                width: 20.0,
                height: 15.5,
            },
            color: Color::Black,
        }
    }
    fn show_box(&self) {
        println!(
            "Box Dimensions\nWidth:{}\nHeight:{}",
            self.dimensions.width, self.dimensions.height
        );
        println!(
            "Box Color: {}\n",
            match self.color {
                Color::Black => "Black",
                Color::White => "White",
            }
        );
    }
}

fn main() {
    let bx1 = ShippingBox::boxv1();
    let bx2 = ShippingBox::boxv2();

    bx1.show_box();
    bx2.show_box();
}
