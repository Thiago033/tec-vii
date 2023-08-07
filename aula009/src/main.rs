struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

pub struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
    tail: Option<*mut Node<T>>,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList {
            head: None,
            tail: None,
        }
    }

    pub fn push(&mut self, data: T) {
        let new_node = Box::new(Node {
            data,
            next: None,
        });

        let raw_node = Box::into_raw(new_node);

        unsafe {
            if let Some(tail) = self.tail {
                (*tail).next = Some(Box::from_raw(raw_node));
            } else {
                self.head = Some(Box::from_raw(raw_node));
            }
            self.tail = Some(raw_node);
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|boxed_node| {
            let node = *boxed_node;
            self.head = node.next;
            if self.head.is_none() {
                self.tail = None;
            }
            node.data
        })
    }
}

fn main() {
    let mut list: LinkedList<i32> = LinkedList::new();

    list.push(1);
    list.push(2);
    list.push(3);

    println!("{:?}", list.pop()); 
    println!("{:?}", list.pop()); 
    println!("{:?}", list.pop()); 
    println!("{:?}", list.pop()); 
}
