```shell
> rustc borrow-display.rs
borrow-display.rs:22:39: 22:44 error: the trait `core::fmt::Display` is not implemented for the type `fn(Tree) -> i32 {depth}` [E0277]
borrow-display.rs:22     println!("The depth of {} is {}", depth, depth(tree));
                                                           ^~~~~
<std macros>:2:25: 2:56 note: in this expansion of format_args!
<std macros>:3:1: 3:54 note: in this expansion of print! (defined in <std macros>)
borrow-display.rs:22:5: 22:59 note: in this expansion of println! (defined in <std macros>)
borrow-display.rs:22:39: 22:44 help: run `rustc --explain E0277` to see a detailed explanation
borrow-display.rs:22:39: 22:44 note: `fn(Tree) -> i32 {depth}` cannot be formatted with the default formatter; try using `:?` instead if you are using a format string
borrow-display.rs:22:39: 22:44 note: required by `core::fmt::Display::fmt`
error: aborting due to previous error
```
