use std::rc::Rc;
use std::cmp::{Ord, Eq};
use std::fmt::Display;
use crate::node::Node;

pub enum Pointer<T: Ord + Eq>{
    Null,
    Point(Rc<Node<T>>)
}

impl<T: Ord + Eq> Pointer<T> {
    pub fn insert(&mut self,date: T,father: Pointer<T>){
        match self {
            //in case the pointer is null, i need to create a new node
            Pointer::Null => {
                *self = Pointer::Point(Rc::new(Node::new(date,father)));
            }
            //otherwise i just pass pass the element to the lower node
            Pointer::Point(e) => {
                e.insert(date,e.clone());
            }
        }
    }
}
//printing methods
impl<T:  Ord + Eq + Display> Pointer<T> {
    pub fn print_order(&self){
        match self {
            Pointer::Point(c) => c.print_order(),
            _ => {}
        }
    }
}
