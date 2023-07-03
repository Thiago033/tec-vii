enum Shape {
    Square(f64),               // side
    Circle(f64),               // radius
    Triangle(f64, f64),        // base and height
    Rectangle(f64, f64),       // base and height
}

fn main() {
    let square = Shape::Square(4.0);
    let circle = Shape::Circle(5.0);
    let triangle = Shape::Triangle(3.0, 6.0);
    let rectangle = Shape::Rectangle(2.0, 8.0);

    calculate_and_print_area(square);
    calculate_and_print_area(circle);
    calculate_and_print_area(triangle);
    calculate_and_print_area(rectangle);
}

fn calculate_and_print_area(shape: Shape) {
    match shape {
        Shape::Square(side) => {
            let area = side * side;
            println!("Area of the square: {}", area);
        },
        Shape::Circle(radius) => {
            let area = 3.14 * radius * radius;
            println!("Area of the circle: {}", area);
        },
        Shape::Triangle(base, height) => {
            let area = 0.5 * base * height;
            println!("Area of the triangle: {}", area);
        },
        Shape::Rectangle(base, height) => {
            let area = base * height;
            println!("Area of the rectangle: {}", area);
        },
    }
}
