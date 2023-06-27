use rand::Rng;

fn gera_vet(size: usize) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    let mut vet = Vec::with_capacity(size);
    for _ in 0..size {
        vet.push(rng.gen_range(0..100));
    }
    vet
}

fn bubble_sort(vet: &mut [i32]) {
    let index = vet.len();

    for i in 0..index {
        let mut swap = false;

        for j in 0..(index - i - 1) {
            if vet[j] > vet[j + 1] {
                vet.swap(j, j + 1);
                swap = true;
            }
        }

        if !swap { break; };
    }
}

fn main() {
    let size = 10;
    let mut vet = gera_vet(size);

    println!("Vetor original: {:?}", vet);

    bubble_sort(&mut vet);

    println!("Vetor ordenado: {:?}", vet);
}