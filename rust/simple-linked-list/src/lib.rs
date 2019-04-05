struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    fn new(data: T, next: Option<Box<Node<T>>>) -> Node<T> {
        Node { data, next }
    }
}

pub struct SimpleLinkedList<T> {
    head: Option<Box<Node<T>>>,
    len: usize,
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        SimpleLinkedList { head: None, len: 0 }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn push(&mut self, element: T) {
        self.len += 1;
        let head = self.head.take();
        self.head = Some(Box::new(Node::new(element, head)));
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.head.is_none() {
            return None;
        }

        self.len -= 1;
        let node = self.head.take().unwrap();
        self.head = node.next;
        Some(node.data)
    }

    pub fn peek(&self) -> Option<&T> {
        match &self.head {
            None => None,
            Some(node) => Some(&node.data),
        }
    }
}

impl<T: Clone> SimpleLinkedList<T> {
    pub fn rev(&self) -> SimpleLinkedList<T> {
        let mut list = SimpleLinkedList::new();
        let mut node = &self.head;
        while let Some(item) = node {
            list.push(item.data.clone());
            node = &item.next;
        }
        list
    }
}

impl<'a, T: Clone> From<&'a [T]> for SimpleLinkedList<T> {
    fn from(items: &[T]) -> Self {
        let mut list = SimpleLinkedList::new();
        items.iter().for_each(|v| list.push(v.clone()));
        list
    }
}

impl<T> Into<Vec<T>> for SimpleLinkedList<T> {
    fn into(mut self) -> Vec<T> {
        let mut list = vec![];
        let mut node = self.head.take();
        while let Some(mut item) = node {
            node = item.next.take();
            list.push(item.data);
        }
        list.reverse();
        list
    }
}
