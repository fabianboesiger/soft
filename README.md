# Soft Assertions

This library provides soft, non-panicking assertions.
Instead, the assertions provided by this crate returns a `Result`.
To use this crate, simply replace `assert!(...)` by `soft::assert!(...)?`.

## Example

```rust
fn main() {
    soft::assert!(true).unwrap();
    soft::assert!(false).unwrap_err();

    soft::assert_eq!(2, 2).unwrap();
    soft::assert_eq!(2, 3).unwrap_err();
    
    soft::assert_ne!(2, 3).unwrap();
    soft::assert_ne!(2, 2).unwrap_err();
}
```
