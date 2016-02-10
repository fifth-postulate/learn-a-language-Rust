enum Tree {
    Empty,
    Leaf(i32),
    Node(Box<Tree>, Box<Tree>)
}

fn max(a: i32, b: i32) -> i32 {
    if a < b {b} else {a}
}

fn depth(tree: Tree) -> i32 {
    match tree {
        Tree::Empty => 0,
        Tree::Leaf(_) => 1,
        Tree::Node(left, right) => max(depth(*left), depth(*right)) + 1
    }
}

fn main() {
    let tree: Tree = Tree::Node(Box::new(Tree::Empty), Box::new(Tree::Leaf(42)));

    println!("The depth of the tree is {}", depth(tree));
}
