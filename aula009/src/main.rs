use std::cmp::PartialEq;

struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

impl<T: PartialEq> PartialEq for Node<T> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
    tail: Option<*mut Node<T>>,
}

impl<T: PartialEq> LinkedList<T> {
    fn new() -> Self {
        LinkedList { head: None, tail: None }
    }

    fn insert(&mut self, value: T) {
        if self.contains(&value) {
            return;
        }

        let new_node = Box::new(Node {
            value,
            next: None,
        });

        let raw_node = Box::into_raw(new_node);

        unsafe {
            if let Some(tail) = self.tail {
                (*tail).next = Some(Box::from_raw(raw_node));
                self.tail = Some(raw_node);
            } else {
                self.head = Some(Box::from_raw(raw_node));
                self.tail = Some(raw_node);
            }
        }
    }

    fn pop(&mut self) -> Option<T> {
        match self.head.take() {
            Some(old_head) => {
                self.head = old_head.next;
                if self.head.is_none() {
                    self.tail = None;
                }
                Some(old_head.value)
            }
            None => None,
        }
    }

    fn contains(&self, value: &T) -> bool {
        let mut current = &self.head;

        while let Some(node) = current {
            if &node.value == value {
                return true;
            }
            current = &node.next;
        }

        false
    }
}

fn main() {
    let mut list: LinkedList<i32> = LinkedList::new();

    list.insert(1);
    list.insert(2);
    list.insert(3);
    list.insert(4);
    list.insert(5);

    println!("List:");

    let mut current = &list.head;

    while let Some(node) = current {
        println!("{}", node.value);
        current = &node.next;
    }

    while let Some(value) = list.pop() {
        println!("Pop Value: {}", value);
    }
}
