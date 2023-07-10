// Definição da estrutura do nó
struct Node {
    data: i32,
    next: Option<Box<Node>>,
}

// Definição da estrutura da lista FIFO
struct Queue {
    head: Option<Box<Node>>,
    tail: *mut Node,
}

impl Queue {
    // Cria uma nova lista FIFO vazia
    fn new() -> Queue {
        Queue {
            head: None,
            tail: std::ptr::null_mut(),
        }
    }

    // Adiciona um elemento no final da lista FIFO
    fn push(&mut self, data: i32) {
        let new_tail = Box::into_raw(Box::new(Node {
            data,
            next: None,
        }));

        unsafe {
            if !self.tail.is_null() {
                (*self.tail).next = Some(Box::from_raw(new_tail));
            } else {
                self.head = Some(Box::from_raw(new_tail));
            }
            self.tail = new_tail;
        }
    }

    // Remove o primeiro elemento da lista FIFO e retorna seu valor
    fn pop(&mut self) -> Option<i32> {
        self.head.take().map(|node| {
            let node = *node;
            self.head = node.next;
            if self.head.is_none() {
                self.tail = std::ptr::null_mut();
            }
            node.data
        })
    }

    // Retorna o primeiro elemento da lista FIFO
    fn get_first(&self) -> Option<i32> {
        self.head.as_ref().map(|node| node.data)
    }

    // Imprime os elementos da lista FIFO
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
    let mut my_queue = Queue::new();
    my_queue.push(1);
    my_queue.push(2);
    my_queue.push(3);

    // Testando a função get_first
    if let Some(first) = my_queue.get_first() {
        println!("Primeiro elemento: {}", first);
    }

    my_queue.print();

    // Testando a função pop
    if let Some(first) = my_queue.pop() {
        println!("Elemento removido: {}", first);
    }

    if let Some(first) = my_queue.get_first() {
        println!("Novo primeiro elemento: {}", first);
    }

    my_queue.print();
}