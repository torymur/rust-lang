use std::cell::RefCell;
use std::rc::Rc;
use std::rc::Weak;

#[derive(Debug)]
struct LinkedList<T> {
    head: Option<Rc<Node<T>>>,
}

#[derive(Debug)]
struct LinkedWeakList<T> {
    head: Option<Rc<WeakNode<T>>>,
}

#[derive(Debug)]
struct LinkedWeakCellList<T> {
    head: Option<Rc<WeakCellNode<T>>>,
}

#[derive(Debug)]
struct Node<T> {
    next: Option<Rc<Node<T>>>,
    data: T,
}

#[derive(Debug)]
struct WeakNode<T> {
    next: Option<Rc<WeakNode<T>>>,
    prev: Option<Weak<WeakNode<T>>>,
    data: T,
}

#[derive(Debug)]
struct WeakCellNode<T> {
    next: Option<Rc<WeakCellNode<T>>>,
    prev: RefCell<Option<Weak<WeakCellNode<T>>>>,
    data: T,
}

impl<T> LinkedList<T> {
    fn new() -> Self {
        LinkedList { head: None }
    }
    fn append(&self, data: T) -> Self {
        LinkedList {
            head: Some(Rc::new(Node {
                data: data,
                next: self.head.clone(),
            })),
        }
    }
}

impl<T> LinkedWeakList<T> {
    fn new() -> Self {
        LinkedWeakList { head: None }
    }
    fn append(&mut self, data: T) -> Self {
        let mut new_node = WeakNode {
            data: data,
            next: self.head.clone(),
            prev: None,
        };
        new_node.prev = match self.head.clone() {
            Some(node) => Some(Rc::downgrade(&node)),
            None => None,
        };
        LinkedWeakList {
            head: Some(Rc::new(new_node)),
        }
    }
}

impl<T> LinkedWeakCellList<T> {
    fn new() -> Self {
        LinkedWeakCellList { head: None }
    }
    fn append(&mut self, data: T) -> Self {
        let new_node = Rc::new(WeakCellNode {
            data: data,
            next: self.head.clone(),
            prev: RefCell::new(None),
        });
        match self.head.clone() {
            Some(node) => {
                let mut prev = new_node.prev.borrow_mut();
                *prev = Some(Rc::downgrade(&node))
            }
            None => {}
        }
        LinkedWeakCellList {
            head: Some(new_node),
        }
    }
}

fn main() {
    let list_of_nums = LinkedList::new().append(1).append(2);
    println!("nums: {:?}", list_of_nums);
    let list_of_strs = LinkedList::new().append("foo").append("bar");
    println!("strs: {:?}", list_of_strs);
    let weak_list_of_nums = LinkedWeakList::new().append(1).append(2).append(3);
    println!("strs: {:?}", weak_list_of_nums);
    let weak_cell_list_of_nums = LinkedWeakCellList::new().append(1).append(2).append(3);
    println!("strs: {:?}", weak_cell_list_of_nums);
}
