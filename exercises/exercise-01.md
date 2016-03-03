# Exercise 1
Continue with the solution of the previous exercise. Or if you want to start
with a clean slate, begin with `exercise-01.rs`.

Change the file to not only print the depth of a tree, but also a representation
of the tree. For example, for the tree `Tree::Node(Box::new(Tree::Empty),
Box::new(Tree::Leaf(42)))` the code

```rust
println!("the depth of {} is {}", tree, depth(tree));
```

should print 

```plain
the depth of (() (42)) is 2
```

Maybe the documentation of [Display][display], [Ownership][ownership] and
[Borrowing][borrowing] will be helpful.

[display]: https://doc.rust-lang.org/std/fmt/trait.Display.html
[ownership]: https://doc.rust-lang.org/book/ownership.html
[borrowing]: https://doc.rust-lang.org/book/references-and-borrowing.html 
