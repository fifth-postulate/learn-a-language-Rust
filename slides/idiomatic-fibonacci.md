```rust
fn fib(n: i64) -> i64 {
    match n {
        0 => 0,
        1 => 1,
        _ => fib(n - 1) + fib(n - 2)
    }
}

fn main() {
    let n = 10;

    println!("fib({}) = {}", n, fib(n));
}
```
