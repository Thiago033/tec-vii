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

// fn sort_rectangles(rectangles_array: &mut Vec<Rectangle>) {
//     rectangles_array.sort_by(|a, b| a.area.cmp(&b.area));
// }

fn insert_rectangle(rectangles_array: &mut Vec<Rectangle>, rectangle: Rectangle) {
    let insert_index = rectangles_array.iter().position(|a| a.area > rectangle.area);

    if let Some(position) = insert_index {
        rectangles_array.insert(position, rectangle);
    } else {
        rectangles_array.push(rectangle);
    }
}

fn main() {
    let mut rectangles_array: Vec<Rectangle> = Vec::new();

    for _ in 0..10 {
        let rectangle = new_rectangle();
        insert_rectangle(&mut rectangles_array, rectangle);
    }

    for rectangle in &rectangles_array {
        println!("{:?}", rectangle);
    }
}
