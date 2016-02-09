enum Tree {
    Empty,
    Leaf(i32),
    Node(Tree, Tree)
}

fn main() {
    let tree: Tree = Tree::Empty;
}
