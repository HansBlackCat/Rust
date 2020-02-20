#![allow(non_snake_case, dead_code)]


#[derive(Debug, Clone)]
pub enum BinaryTree {
    Node {
        value: i32,
        leftTree: Box<BinaryTree>,
        rightTree: Box<BinaryTree>,
    },
    Empty,
}


impl BinaryTree {
    pub fn singleton(x: i32) ->BinaryTree {
        BinaryTree::Node {
            value: x,
            leftTree: Box::new(BinaryTree::Empty),
            rightTree: Box::new(BinaryTree::Empty),
        }
    }

    pub fn simple_print(&self) ->() {
        println!("{:#?}", &self);
    }

    pub fn find(&self, x: i32) ->Option<i32> {
        match self {
            BinaryTree::Empty  => None,
            BinaryTree::Node {
                value: v,
                leftTree: _,
                rightTree: _,
            } if *v==x         => Some(*v),
            BinaryTree::Node {
                value: v,
                leftTree: _,
                rightTree: r,
            } if *v<x          => r.find(x),
            BinaryTree::Node {
                value: v,
                leftTree: l,
                rightTree: _,
            } if *v>x          => l.find(x),
            _                  => panic!("unexpected matching"),
        }
    }

    pub fn insert(&self, x: i32) ->BinaryTree {
        match self {
            BinaryTree::Empty  => BinaryTree::singleton(x),
            BinaryTree::Node {
                value: v,
                leftTree: l,
                rightTree: r,
            } if *v==x        => {
                BinaryTree::Node {
                    value: *v,
                    leftTree: l.clone(),
                    rightTree: r.clone(),
                }
            },
            BinaryTree::Node {
                value: v,
                leftTree: l,
                rightTree: r,
            } if *v<x         => {
                BinaryTree::Node {
                    value: *v,
                    leftTree: l.clone(),
                    rightTree: Box::new(r.insert(x)),
                }
            }
            BinaryTree::Node {
                value: v,
                leftTree: l,
                rightTree: r,
            } if *v>x         => {
                BinaryTree::Node {
                    value: *v,
                    leftTree: Box::new(l.insert(x)),
                    rightTree: r.clone(),
                }
            }
            _                 => panic!("unexpected mathcing"),
        }
    }

    pub fn from_vec(_x: &Vec<i32>) ->BinaryTree {
        let x = _x.clone();
        let mut init: BinaryTree = BinaryTree::Empty;
        for i in x.iter() {
            init = init.insert(*i);
        }
        init
    }

}
