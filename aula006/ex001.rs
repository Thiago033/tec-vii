use std::io;

fn age_group(age: i32) {
    if age <= 2 {
        println!("Bebê");
    } else if age <= 12 {
        println!("Criança");
    } else if age < 19 {
        println!("Adolescente");
    } else if age <= 29 {
        println!("Jovem adulto");
    } else if age <= 39 {
        println!("Adulto jovem");
    } else if age <= 59 {
        println!("Adulto de meia-idade");
    } else {
        println!("Idoso");
    }
}

fn main() {
    println!("Enter an integer:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let number: i32 = input.trim().parse().expect("Invalid input. Please enter an integer.");

    age_group(number);
}
