use std::mem;

pub struct Node {
    pub elem: i32,
    pub next: Link
}

pub enum Link {
    Empty,
    More(Box<Node>)
}

pub struct List {
    pub head: Link,
}

impl List {
    pub fn push(&mut self, elem: i32) {
        let new_node = Box::new(Node {
            elem,
            next: mem::replace(&mut self.head, Link::Empty)
        });
        self.head = Link::More(new_node);
    }
    
    pub fn pop(&mut self) -> Option<i32> {
        let result;
        match &self.head {
            Link::Empty => {
                result = None;
            },
            Link::More(node) => {
                result = Some(node.elem);
                self.head = node.next;
            }
        };
        return result;
    }
}

