# Excercise 0

Do not forget about the excellent [documentation][] or `rustc --explain`. 

## Make Tree compile

Starting from the following file

```rust
enum Tree {
    Empty,
    Leaf(i32),
    Node(Tree, Tree)
}

fn main() {
    let tree: Tree = Tree::Empty;
}
```

make it compile, keeping the spirit of `Tree` intact.

## Create a `depth` function

Extend the previous code to include a `depth` function. This function should
determine the depth of an arbitrary tree. The depth of the `Empty` tree is
zero, the depth of a `Leaf` is one and the depth of a `Node` is one more than
the maximum of the depth of her sub-trees.

[documentation]: https://www.rust-lang.org/ 
