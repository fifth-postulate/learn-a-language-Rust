```shell
> rustc naive-algebraic-datatype.rs
naive-algebraic-datatype.rs:1:1: 5:2 error: invalid recursive enum type [E0072]
naive-algebraic-datatype.rs:1 enum Tree {
naive-algebraic-datatype.rs:2     Empty,
naive-algebraic-datatype.rs:3     Leaf(i32),
naive-algebraic-datatype.rs:4     Node(Tree, Tree)
naive-algebraic-datatype.rs:5 }
naive-algebraic-datatype.rs:1:1: 5:2 help: run `rustc --explain E0072` to see a detailed explanation
naive-algebraic-datatype.rs:1:1: 5:2 help: wrap the inner value in a box to make it representable
error: aborting due to previous error
```
