#![deny(clippy::all, clippy::pedantic)]

struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    fn new(data: T, next: Option<Box<Self>>) -> Self {
        Self { data, next }
    }
}

#[derive(Default)]
pub struct SimpleLinkedList<T> {
    head: Option<Box<Node<T>>>,
    len: usize,
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        Self { head: None, len: 0 }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn push(&mut self, element: T) {
        self.len += 1;
        let head = self.head.take();
        self.head = Some(Box::new(Node::new(element, head)));
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.as_ref()?;

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
    pub fn rev(&self) -> Self {
        let mut list = Self::new();
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
        let mut list = Self::new();
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
