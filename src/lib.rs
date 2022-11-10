use std::cell::RefCell;
use std::rc::Rc;
use std::cmp::{Ord, Eq};

#[cfg(test)]
pub mod test;

enum Elem<T: Ord + Eq>{
    Null,
    Point(Rc<RefCell<Node<T>>>)
}



struct Node<T: Ord + Eq>{
    date: T,
    father: Elem<T>,
    left: Elem<T>,
    right: Elem<T>
}
impl<T: Ord + Eq> Node<T>{
    fn inert(&mut self){

    }
}

impl<T: Ord + Eq> Node<T>{
    fn new(date: T)->Node<T>{
        Node{
            date,
            father: Elem::Null,
            left: Elem::Null,
            right: Elem::Null,
        }
    }
}


struct Tree<T: Ord + Eq>{
    root: Elem<T>
}

impl<T: Ord + Eq> Tree<T>{
    fn new()->Tree<T>{
        Tree{
            root: Elem::Null
        }
    }

    fn insert(&mut self, date: T){
        match self.root {
            Elem::Null => {
                self.root = Elem::Point(
                    Rc::new(
                        RefCell::new(
                            Node::new(date)
                        )
                    )
                )
            }
            Elem::Point(e):
        }
    }


}