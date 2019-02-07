use std::mem;

pub struct List {
    head: Link,
}

enum Link {
    Empty,
    More(Box<Node>),
}

struct Node {
    elem: i32,
    next: Link,
}

impl List {
    pub fn new() -> Self {
        List {
            head: Link::Empty
        }
    }

    // When List is new
    // - new_node.next = self.head (which is currently Empty)
    //      - new_node.next = Empty
    // - self.head (which is currently Empty) = Empty
    //      - self.head = Empty
    // - self.head = new_node
    //
    // When List has element(s)
    // - new_node.next = self.head (which is currently a Node)
    //      - new_node.next = Node
    // - self.head (which is currently a Node) = Empty
    //      - self.head = Empty
    // - self.head = new_node
    pub fn push(&mut self, elem: i32) {
        let new_node = Box::new(Node {
            elem,
            next: mem::replace(&mut self.head, Link::Empty),
        });

        self.head = Link::More(new_node);
    }

    // If it is Empty
    // - just return None
    // If it is not Empty
    // - remove the head of the list
    // - remove its element
    // - replace the list head with its next
    // - return the element
    pub fn pop(&mut self) -> Option<i32> {
        let boxed_head_node = mem::replace(&mut self.head, Link::Empty);
        match boxed_head_node {
            Link::Empty => None,
            Link::More(boxed_head_node) => {
                self.head = boxed_head_node.next;
                Some(boxed_head_node.elem)
            },
        }
    }
}

impl Drop for List {
    fn drop(&mut self) {
        let mut cur_link = mem::replace(&mut self.head, Link::Empty);
        while let Link::More(mut boxed_node) = cur_link {
            cur_link = mem::replace(&mut boxed_node.next, Link::Empty);
        }
    }
}

mod test {
    use super::*;

    #[test]
    fn basics() {
        let mut list = List::new();

        assert_eq!(list.pop(), None);

        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        list.push(4);
        list.push(5);

        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));

        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }
}
