use crate::Tree;

#[test]
fn main_test(){
    println!("it works!");

    let mut tree = Tree::<i32>::new();

    tree.insert(1);
    tree.insert(4);
    tree.insert(12);
    tree.insert(2);

    tree.print_order();

}