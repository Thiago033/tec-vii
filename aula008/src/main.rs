#[derive(Clone)]
struct Pessoa {
    nome: String,
    idade: u32,
}
fn main() {
    let pessoa1 = Pessoa {
        nome: String::from("Gerson"),
        idade: 30,
    };
    let pessoa2 = pessoa1.clone();
    println!("pessoa1.nome = {}, pessoa1.idade = {}", pessoa1.nome, pessoa1.idade);
    println!("pessoa2.nome = {}, pessoa2.idade = {}", pessoa2.nome, pessoa2.idade);
}