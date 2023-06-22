fn main() {

    let array:[i64; 5] = [1, 2, 3, 4, 5];

    let mut array_sum: i64 = 0;

    for i in 0..array.len() {
        array_sum = array_sum + array[i];
    }

    println!("{array_sum}");
}
