use std::borrow::BorrowMut;
use std::cell::{Ref, RefCell};
use std::rc::Rc;
use std::cmp::{Ord, Eq};
use std::fmt::Display;

#[cfg(test)]
mod test;
mod pointer;
mod node;

use pointer::Pointer;
use node::{InnerNode, Node};

pub struct Tree<T: Ord + Eq>{
    root: Pointer<T>
}

impl<T: Ord + Eq> Tree<T> {
    pub fn insert(&mut self, date: T){
        //insering data into pointer with Null father (because is the root)
        self.root.insert(date, Pointer::Null)
    }
    pub fn new()->Self{
        Self{
            root: Pointer::Null
        }
    }
}

//printing methods
impl<T: Ord + Eq + Display> Tree<T> {
    pub fn print_order(&self){
        self.root.print_order();
    }
}