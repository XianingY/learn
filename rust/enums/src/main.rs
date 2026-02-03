enum Shape {
    Circle(f64),
    Rectangle(f64, f64),
}

impl Shape {
    fn area(&self) -> f64 {
        match self {
            Shape::Circle(r) => 3.14 * r * r,
            Shape::Rectangle(w, h) => w * h,
        }
    }
}

fn main() {
    let c = Shape::Circle(5.0);
    let r = Shape::Rectangle(3.0, 4.0);

    println!("Circle area: {:.1}", c.area());
    println!("Rectangle area: {:.1}", r.area());
}
