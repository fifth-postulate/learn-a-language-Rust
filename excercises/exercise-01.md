# Exercise 1
Continue with the solution of the previous exercise. Or if you want to start
with a clean start, begin with `exercise.01.rs`.

Change the file to not only print the depth of a tree, but also representation
of the tree. For example, for the tree `Tree::Node(Box::new(Tree::Empty),
Box::new(Tree::Leaf(42)))` it should print

```plain
the depth of (() (42)) is 2
```