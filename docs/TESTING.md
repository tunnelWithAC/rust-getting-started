In Rust, you can run unit tests using the built-in test framework. Here's how you can do it:

### Write Tests

In Rust, unit tests are typically written in the same file as the code they're testing, inside a `mod tests` block annotated with `#[cfg(test)]`. Here's an example:

```rust
// Function to test
pub fn add_two(a: i32) -> i32 {
    a + 2
}

// Unit tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_two() {
        assert_eq!(add_two(2), 4);
    }
}
```

In this example, `test_add_two` is a test function that tests the `add_two` function. The `#[test]` attribute tells Rust this is a test function.

### Run Tests

You can run your tests by executing the command `cargo test` in your terminal. This will compile your code with the test configuration and run all test functions.

Remember, test functions in Rust should take no parameters and return nothing. If a test function completes without panicking, it's considered to have passed. If it does panic, it's considered to have failed.