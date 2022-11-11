use std::cell::RefCell;
use std::cmp::{Ord, Eq};
use std::fmt::Display;
use std::rc::Rc;
use crate::Pointer::{Null, Point};
use crate::pointer::Pointer;


pub struct Node<T: Ord + Eq>{
    inner: RefCell<InnerNode<T>>
}
impl<T: Ord + Eq> Node<T> {
    pub fn insert(&self,date: T,this_node_RC: Rc<Node<T>>){
        self.inner.borrow_mut().insert(date,this_node_RC);
    }
    //when i create a new node i insert the data and the father...
    //dx and sx child are initialized tp null
    pub fn new(date: T,father: Pointer<T>)->Self{
        Self{
            inner: RefCell::new(InnerNode::new(date,father))
        }
    }

}

//printing methods
impl<T:  Ord + Eq + Display> Node<T> {
    pub fn print_order(&self){
        self.inner.borrow().print_order();
    }
}


pub struct InnerNode<T: Ord + Eq>{
    father: Pointer<T>,
    dx: Pointer<T>,
    sx: Pointer<T>,
    date: T

}
impl<T: Ord + Eq> InnerNode<T> {
    pub fn insert(&mut self,date: T,this_node_Rc: Rc<Node<T>>){
        //smaller or equal elements to the left
        if date <= self.date{
            self.sx.insert(date,Pointer::Point(this_node_Rc))
        }else {
            self.dx.insert(date,Pointer::Point(this_node_Rc))
        }
    }
    pub fn new(date: T, father: Pointer<T>)-> Self{
        Self{
            father,
            dx: Pointer::Null,
            sx: Pointer::Null,
            date
        }
    }
}

//printing methods
impl<T: Ord + Eq + Display> InnerNode<T> {
    pub fn print_order(&self){
        self.sx.print_order();
        println!("{}",self.date);
        self.dx.print_order();
    }
}
