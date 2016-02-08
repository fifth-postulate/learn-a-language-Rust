enum Tree {
    Empty,
    Leaf(i32),
    Node(Box<Tree>, Box<Tree>)
}

impl std::fmt::Display for Tree {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Tree::Empty => write!(f, "()"),
            Tree::Leaf(x) => write!(f, "({})", x),
            Tree::Node(ref left, ref right) => write!(f, "({} {})", *left, *right)
        }
    }
}

fn max(x: i32, y: i32) -> i32 {
    if x < y { y } else { x }
}

fn depth(tree: &Tree) -> i32 {
    match *tree {
        Tree::Empty   => 0,
        Tree::Leaf(_) => 1,
        Tree::Node(ref left, ref right) => max(depth(&(*left)), depth(&(*right))) + 1
    }
}

fn main() {
    let tree: Tree = Tree::Node(Box::new(Tree::Empty), Box::new(Tree::Leaf(42)));

    println!("tree {} has depth {}", tree, depth(&tree));
}
