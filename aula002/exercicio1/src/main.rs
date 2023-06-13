use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let x: i32 = args[1].trim().parse().expect("Input not an integer");

    for i in 1..= x {

        for _j in 0..i {
            print!("*");
        }

        println!();
    }
}
