fn compare_swap(x: &mut i32, y: &mut i32) {
    let temp = *x;
    *x = *y;
    *y = temp;
}

fn main() {
    
    let mut x: i32 = 5;
    let mut y: i32 = 10;
    
    swap(&mut x, &mut y);

    println!("X: {:?} | Y: {:?}", x, y);
}