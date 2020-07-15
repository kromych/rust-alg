use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
pub struct LinkedList<T>(Option<(T, Box<LinkedList<T>>)>);

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList(None)
    }

    pub fn push_front(&mut self, data: T) {
        let t = self.0.take();
        self.0 = Some((data, Box::new(LinkedList(t))));
    }

    pub fn push_back(&mut self, data: T) {
        match self.0 {
            Some((_, ref mut child)) => child.push_back(data),
            None => self.push_front(data)
        }
    }
}

pub struct DlNode<T> {
    data: T,
    next: Option<Rc<RefCell<DlNode<T>>>>,
    prev: Option<Weak<RefCell<DlNode<T>>>>
}

pub struct DlList<T> {
    first: Option<Rc<RefCell<DlNode<T>>>>,
    last: Option<Weak<RefCell<DlNode<T>>>>
}

impl<T> DlList<T> {
    pub fn new() -> Self {
        DlList {
            first: None,
            last: None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linked_list() {
        let mut ll = LinkedList::new();

        ll.push_front(3);
        ll.push_back(12);
        ll.push_back(8);

        println!("{:?}", ll);
    }
}
