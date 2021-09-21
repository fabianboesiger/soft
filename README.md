# Soft Assertions

This library provides soft, non-panicking assertions.
Instead, the assertions provided by this crate returns a `Result`.
To use this crate, simply replace `assert!(...)` by `soft::assert!()?`.