struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    fn new(data: T) -> Node<T> {
        Node { data, next: None }
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
        let element = Some(Box::new(Node::new(element)));

        if self.head.is_none() {
            self.head = element;
            return;
        }

        let mut node = self.head.as_mut().unwrap();
        while node.next.is_some() {
            node = node.next.as_mut().unwrap();
        }
        node.next = element;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.head.is_none() {
            return None;
        }

        self.len -= 1;

        if self.len == 0 {
            let head = self.head.take();
            return Some(head.unwrap().data);
        }

        let mut node = self.head.as_mut().unwrap();
        while node.next.as_ref().unwrap().next.is_some() {
            node = node.next.as_mut().unwrap();
        }
        let tail = node.next.take();
        Some(tail.unwrap().data)
    }

    pub fn peek(&self) -> Option<&T> {
        if self.len == 0 {
            return None;
        }

        let mut node = self.head.as_ref().unwrap();
        while node.next.is_some() {
            node = node.next.as_ref().unwrap();
        }
        Some(&node.data)
    }
}

impl<T: Clone> SimpleLinkedList<T> {
    pub fn rev(&self) -> SimpleLinkedList<T> {
        let mut items = vec![];
        let mut node = &self.head;
        while let Some(item) = node {
            items.push(item.data.clone());
            node = &item.next;
        }

        let mut list = SimpleLinkedList::new();
        while !items.is_empty() {
            list.push(items.pop().unwrap())
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
        list
    }
}
