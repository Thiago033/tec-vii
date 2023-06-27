fn main() {
    let size = 3;
    let mut matrix = vec![vec![0; size]; size];

    for i in 0..size {
        matrix[i][i] = 0;
    }
    
    for i in 0..size {
        for j in i..size {
            matrix[i][j] = 1;
        }
    }
    
    for i in 0..size {
        for j in 0..i {
            matrix[i][j] = -1;
        }
    }

    for i in 0..size {
        for j in 0..size {
            print!("{} ", matrix[i][j]);
        }
        println!();
    }
}
