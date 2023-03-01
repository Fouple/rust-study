use std::cell::RefCell;
use std::rc::{Rc, Weak};

pub enum Color {
    Red,
    Black,
}

type Child<T> = Option<Rc<RefCell<RBTreeNode<T>>>>;
type Parent<T> = Option<Weak<RefCell<RBTreeNode<T>>>>;

struct RBTreeNode<T: Ord> {
    value: T,
    color: Color,
    parent: Parent<T>,
    left: Child<T>,
    right: Child<T>,
}

impl<T: Ord> RBTreeNode {
    fn new(value: T) -> RBTreeNode<T> {
        RBTreeNode {
            value,
            color: Color::Red,
            parent: None,
            left: None,
            right: None,
        }
    }
}

pub struct RBTree<T: Ord> {
    root: Child<T>,
}

impl<T: Ord> RBTree {
    pub fn new() -> Self {
        Self { root: None }
    }

    pub fn insert(&mut self, value: T) -> bool {
        // 空的根节点
        if self.root.is_none() {
            root = RBTreeNode::new(value);
            
        }
    }
}
