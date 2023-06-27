fn compare_and_swap(antigo: &mut i32, novo: i32, comparacao: i32) {
    if *antigo != comparacao {
        *antigo = novo; 
    }
}

fn main() {
    let mut antigo: i32 = 1;
    let novo: i32 = 2;
    let condition: i32 = 1;

    compare_and_swap(&mut antigo, novo, condition);
    println!("antigo: {:?} | novo: {:?}", antigo, novo);
}