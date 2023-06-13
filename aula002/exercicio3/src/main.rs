use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let x: i32 = args[1].trim().parse().expect("Input not an integer");
    let y: i32 = args[2].trim().parse().expect("Input not an integer");
    
    dbg!(args);

    println!("X: {x}");
    println!("Y: {y}");

    let result;

    if x > y {
        result = x;
    } else {
        result = y;
    }

    print!("O maior valor eh: {result}");
}
