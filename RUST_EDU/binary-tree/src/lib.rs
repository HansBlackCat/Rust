mod binary_tree;

#[cfg(test)]
mod tests {
    #[test]
    fn main() {
        use crate::binary_tree::*;
        println!("Testttttttttttttt");
        BinaryTree::Empty.insert(4).simple_print();
        BinaryTree::singleton(10).insert(3).simple_print();
        BinaryTree::from_vec(&vec![9]).simple_print();
    }
}

