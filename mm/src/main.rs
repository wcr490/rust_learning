// pub enum List {
//     Empty,
//     More(Box<Node>), }
//
// struct Node {
//     Next: List,
//     ele: i32,
// }
//
pub struct List<T> {
    head: Link<T>,
}
type Link<T> = Option<Box<Node<T>>>;
// enum Link {
//     Empty,
//     More(Box<Node>),
// }
struct Node<T> {
    next: Link<T>,
    ele: T,
}

impl<T> List<T> {
    fn new() -> Self {
        List { head: None }
    }

    fn push(&mut self, ele: T) {
        let new_node = Box::new(Node {
            ele,
            next: self.head.take(),
        });
        self.head = Some(new_node);
    }
    fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.ele
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.ele)
    }
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.ele)
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
    pub fn iter<'a>(&'a self) -> Iter<'a, T> {
        Iter {
            next: self.head.as_ref().map(|node| &*node),
        }
    }
    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut {
            next: self.head.as_mut().map(|node| &mut *node),
        }
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut cur_link = std::mem::replace(&mut self.head, None);
        while let Some(boxed_node) = cur_link {
            cur_link = boxed_node.next;
        }
    }
}

pub struct IntoIter<T>(List<T>);
impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

pub struct Iter<'a, T> {
    next: Option<&'a Box<Node<T>>>,
}
impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_ref().map(|node| &*node);
            &node.ele
        })
    }
}

pub struct IterMut<'a, T> {
    next: Option<&'a mut Box<Node<T>>>,
}
impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_mut().map(|node| &mut *node);
            &mut node.ele
        })
    }
}

fn main() {
    let mut l = List::new();
    l.push(5);
    l.push(2);
    l.push(4);
    let mut iter = l.iter();
    assert_eq!(iter.next(), Some(&4));
    assert_eq!(iter.next(), Some(&2));
    for ele in l.iter_mut() {
        *ele += 1;
    }
    for ele in l.iter() {
        println!("{}", ele);
    }
}

#[cfg(test)]
mod test {
    use crate::List;

    #[test]
    fn basics() {
        let mut list = List::new();
        assert_eq!(list.pop(), None);
        list.push(1);
        list.push(100);
        list.push(5);
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.peek(), Some(&100));
        list.peek_mut().map(|val| {
            *val += 100;
        });
        assert_eq!(list.pop(), Some(200));
        list.pop();
        assert_eq!(list.pop(), None);
    }
    #[test]
    fn iter() {
        let mut l = List::new();
        l.push(5);
        l.push(2);
        l.push(4);
        let mut iter = l.iter();
        assert_eq!(iter.next(), Some(&4));
        assert_eq!(iter.next(), Some(&2));
        for ele in iter {
            println!("{}", ele);
        }
    }
}
