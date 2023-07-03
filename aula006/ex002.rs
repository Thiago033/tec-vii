enum Shape {
    Square(f64),               // side
    Circle(f64),               // radius
    Rectangle(f64, f64),       // base and height
    Triangle(f64, f64),        // base and height
}

use Shape::*;

fn main() {
    // let square = Shape::Square(10.0);
    // let circle = Shape::Circle(10.0);
    // let triangle = Shape::Triangle(10.0, 10.0);
    // let rectangle = Shape::Rectangle(10.0, 10.0);

    let square = Square(10.0);
    let circle = Circle(10.0);
    let rectangle = Rectangle(10.0, 10.0);
    let triangle = Triangle(10.0, 10.0);

    calculate_area(square);
    calculate_area(circle);
    calculate_area(rectangle);
    calculate_area(triangle);
}

fn calculate_area(shape: Shape) {
    match shape {
        Shape::Square(side) => {
            println!("area of the square: {}", (side * side));
        },
        Shape::Circle(radius) => {
            println!("area of the cicle: {}", (3.14 * (radius * radius)));
        },
        Shape::Rectangle(base, height) => {
            println!("area of the rectangle: {}", (base * height));
        },
        Shape::Triangle(base, height) => {
            println!("area of the triangle: {}", (0.5 * (base * height)));
        },
    }
}
