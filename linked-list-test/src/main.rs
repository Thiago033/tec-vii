#[derive(Clone)]
enum Address {
    Address(Box<Node>),
    Null,
}

#[derive(Clone)]
struct Node {
    value: u32,
    next: Address,
}

impl Node {
    fn append(&mut self, elem: u32) {
        match self.next {
            Address::Address(ref mut next_address) => {
                next_address.append(elem);
            }
            Address::Null => {
                let node = Node {
                    value: elem,
                    next: Address::Null,
                };
                self.next = Address::Address(Box::new(node))
            }
        }
    }

    fn delete(&mut self, elem: u32) {
        match self.next {
            Address::Address(ref mut next_address) => {
                if next_address.value == elem {
                    println!("Deleting value {}", next_address.value);
                    self.next = next_address.next.clone();
                } else {
                    next_address.delete(elem);
                }
            }
            Address::Null => {
                if self.value == elem {
                    self.value = 0;
                } else {
                    println!("Element {} does not exist in the linked list", elem);
                }
            }
        }
    }

    fn count(&self) -> u32 {
        match self.next {
            Address::Address(ref newaddress) => 1 + newaddress.count(),
            Address::Null => 0,
        }
    }

    fn list(&self) {
        if self.value == 0 {
            println!("The list is empty")
        } else {
            println!("{}", self.value);
            match self.next {
                Address::Address(ref next_address) => next_address.list(),
                Address::Null => {}
            }
        }
    }

    fn update(&mut self, index: u32, elem: u32) {
        let mut i = 0;
        let mut j = self;
        if i == index {
            j.value = elem;
        }
        while i < index {
            // println!("{}", j.value);
            match j.next {
                Address::Address(ref mut next_address) => j = next_address,
                Address::Null => {}
            }
            i = i + 1;
        }
        j.value = elem;
    }
}

fn main() {
    let mut head = Node {
        value: 8,
        next: Address::Null,
    };

    head.append(9);
    head.append(10);
    head.append(11);
    head.update(0, 6);
    head.update(1, 20);


    head.list();
    println!("The size of the list is {}", head.count());

    head.delete(11);

    head.list();
    println!("The size of the list is {}", head.count());

}