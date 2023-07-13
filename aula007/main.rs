// Definição da estrutura do nó da lista encadeada LIFO
// data: dado armazenado (um valor inteiro)
// next:  ponteiro para o proximo nó
struct Node {
    data: i32,
    next: Option<Box<Node>>,
}

impl Node {
    // Cria um novo nó
    // data é inicializado com o valor informado
    // next, por enquanto, não aponta para ninguém
    fn new(data: i32) -> Node {
        Node {
            data: data,
            next: None,
        }
    }
}

// Definição da estrutura da lista encadeada
// A lista, em sí, possui um ponteiro para o início da lista
// head é uma enumeração que pode possuir um dado válido ou None
// None representa um não-valor, algo como NULL.
// Um dado válido é representado por Some(um_tipo_qualquer).
// No caso, Some(<Box<Node>>), ou seja, um ponteiro para um
// Node no heap.
struct Stack {
    head: Option<Box<Node>>,
}

impl Stack {
    // Cria uma nova lista encadeada vazia
    // Inicialmente, o ponteiro head possui None
    fn new() -> Stack {
        Stack { head: None }
    }

    // Adiciona um elemento no início da lista
    // Cria no heap um novo nodo na lista.
    // O novo nó é colocado na cabeça e passa a apontar
    // para o restante da lista.
    // (1) new_node é alocado no heap, tendo o campo data
    //     inicializado neste momento.
    // (2) o método take em uma estrutura permite que o valor
    //     atual de uma variável, no caso head, seja retornado
    //     ao mesmo tempo em que passa a ser None
    //     -->> Isso é bem interessante, pois, na prática,
    //          a lista tem nova cabeça! Lá em C não é assim!
    // (3) self.head passa a apontar para o nodo que foi inserido
    fn push(&mut self, data: i32) {
        let mut new_node = Box::new(Node::new(data)); // (1)
        new_node.next = self.head.take(); // (2)
        self.head = Some(new_node); // (3)
    }

    // Remove o primeiro elemento da lista
    // self.head é do tipo Option<Box<node>> que remove
    // o elemento que está no topo da pilha
    // (1) Retorna o primeiro nodo da lista e torna seu
    //     valor inválido
    // (2) A função map é explicada lá embaixo. Neste ponto
    //     é instanciada uma variável local "node" que possui
    //     o atual primeiro elemento da lista
    // (3) A cabeça da lista é atualizada, fazendo com que
    //     head aponte para o next da cabeça que está sendo
    //     removida
    fn pop(&mut self) {
        self.head.take()     // (1)
                        .map(|mut node| {   // (2)
                          self.head = node.next.take();
                         });
    }

    // Retorna o primeiro elemento da lista
    // (1) o método as_ref retorna uma referência imutável
    //     ao objeto
    // (2) a função map recebe como parâmetro o nó que está
    //     na cabeça da lista e retorna o campo data
    fn get_first(&self) -> Option<i32> {
        self.head.as_ref() // (1)
                          .map(|node| node.data)
    }

    // Imprime os elementos da lista
    fn print(&self) {
        let mut current = &self.head;
        while let Some(node) = current {
            println!("{}", node.data);
            current = &node.next;
        }
    }
}

// Função principal
fn main() {
    let mut my_stack = Stack::new();
    my_stack.push(3);
    my_stack.push(2);
    my_stack.push(1);

    // Testando a função get_first
    if let Some(first) = my_stack.get_first() {
        println!("Primeiro elemento: {}", first);
    }

    my_stack.print();

    // Testando a função pop
    my_stack.pop();

    if let Some(first) = my_stack.get_first() {
       println!("Primeiro elemento: {}", first);
    }

    my_stack.print();
}


//Exemplo simples de map:
fn ex_simples_map() {
    let x: Option<i32> = Some(5);

    // O método map recebe um tipo T de uma Option<T>
    // Este parâmetro está explícito entre ||. No caso,
    // o parâmetro é um i32. Depois dos || existe o corpo
    // de uma função anônima. Esta função retorna algo. Este
    // algo é também do tipo i32.
    let x_quadrado = x.map(|n| n * 2);

    println!("{:?}", x_quadrado);
}
