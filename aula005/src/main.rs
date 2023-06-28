use rand::Rng;

#[derive(Debug)]
struct Rectangle {
    height: i64,
    width: i64,
    area: i64
}

impl Rectangle {
    fn calculate_area(&mut self) {
        self.area = self.height * self.width;
    }
}

fn new_rectangle() -> Rectangle {
    let mut rng = rand::thread_rng();

    let height = rng.gen_range(2..=20);
    let width = rng.gen_range(2..=20);

    let mut rectangle = Rectangle {
        height,
        width,
        area: 0
    };

    rectangle.calculate_area();

    //returns rectangle
    rectangle
}

fn sort_rectangles(rectangles_array: &mut Vec<Rectangle>) {
    rectangles_array.sort_by(|a, b| a.area.cmp(&b.area));
}

fn main() {
    let mut rectangles_array: Vec<Rectangle> = Vec::new();

    for _ in 0..10 {
        rectangles_array.push(new_rectangle());
    }

    //Before sorting
    // for rectangle in &rectangles_array {
    //     println!("{:?}", rectangle);
    // }

    sort_rectangles(&mut rectangles_array);

    //After sorting
    for rectangle in &rectangles_array {
        println!("{:?}", rectangle);
    }
}
