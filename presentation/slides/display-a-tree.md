```rust
let tree: Tree = Tree::Node(Box::new(Tree::Empty), Box::new(Tree::Leaf(42)));

println!("the depth of {} is {}", tree, depth(tree));
```
