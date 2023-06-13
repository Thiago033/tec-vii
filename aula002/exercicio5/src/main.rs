use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let mut array = [0; 10];

    for i in 0..10 {
        array[i] = rng.gen_range(1..10);
    }

    println!("Array: {:?}", array);
}
