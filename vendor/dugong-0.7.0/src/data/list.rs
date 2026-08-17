use serde::Serialize;
use std::cell::RefCell;
use std::fmt;
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct ListInner<T> {
    head: Option<Rc<RefCell<Node<T>>>>,
    tail: Option<Rc<RefCell<Node<T>>>>,
}

impl<T> Default for ListInner<T> {
    fn default() -> Self {
        Self {
            head: None,
            tail: None,
        }
    }
}

#[derive(Debug)]
pub struct Node<T> {
    pub value: T,
    prev: Option<Weak<RefCell<Node<T>>>>,
    next: Option<Rc<RefCell<Node<T>>>>,
    list: Option<Weak<RefCell<ListInner<T>>>>,
}

impl<T> Node<T> {
    pub fn new(value: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            value,
            prev: None,
            next: None,
            list: None,
        }))
    }
}

#[derive(Clone, Debug)]
pub struct List<T> {
    inner: Rc<RefCell<ListInner<T>>>,
}

impl<T> Default for List<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> List<T> {
    pub fn new() -> Self {
        Self {
            inner: Rc::new(RefCell::new(ListInner::default())),
        }
    }

    pub fn enqueue(&self, node: Rc<RefCell<Node<T>>>) {
        detach(&node);

        {
            let mut n = node.borrow_mut();
            n.list = Some(Rc::downgrade(&self.inner));
            n.prev = self.inner.borrow().tail.as_ref().map(Rc::downgrade);
            n.next = None;
        }

        let mut inner = self.inner.borrow_mut();
        if let Some(tail) = inner.tail.take() {
            tail.borrow_mut().next = Some(node.clone());
            inner.tail = Some(node);
            inner.head.get_or_insert(tail);
        } else {
            inner.head = Some(node.clone());
            inner.tail = Some(node);
        }
    }

    pub fn dequeue(&self) -> Option<Rc<RefCell<Node<T>>>> {
        let head = self.inner.borrow().head.clone()?;
        detach(&head);
        Some(head)
    }

    pub fn is_empty(&self) -> bool {
        self.inner.borrow().head.is_none()
    }
}

fn detach<T>(node: &Rc<RefCell<Node<T>>>) {
    let list = node.borrow().list.as_ref().and_then(|w| w.upgrade());

    let Some(list) = list else {
        let mut n = node.borrow_mut();
        n.prev = None;
        n.next = None;
        n.list = None;
        return;
    };

    let (prev, next) = {
        let n = node.borrow();
        (n.prev.clone(), n.next.clone())
    };

    {
        let mut list = list.borrow_mut();

        if let Some(prev) = prev.as_ref().and_then(|w| w.upgrade()) {
            prev.borrow_mut().next = next.clone();
        } else {
            list.head = next.clone();
        }

        if let Some(next) = next.as_ref() {
            next.borrow_mut().prev = prev.clone();
        } else {
            list.tail = prev.and_then(|w| w.upgrade());
        }
    }

    let mut n = node.borrow_mut();
    n.prev = None;
    n.next = None;
    n.list = None;
}

impl<T> fmt::Display for List<T>
where
    T: Serialize,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut parts: Vec<String> = Vec::new();
        let mut cur = self.inner.borrow().head.clone();
        while let Some(node) = cur {
            let json = serde_json::to_string(&node.borrow().value).map_err(|_| fmt::Error)?;
            parts.push(json);
            cur = node.borrow().next.clone();
        }
        write!(f, "[{}]", parts.join(", "))
    }
}
