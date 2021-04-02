/**
## Print debugging information at development time.
[See the manual](https://crates.io/crates/develop_debug)
*/
#[macro_export]
macro_rules! develop_debug {
    ($($token:tt)*) => {};
}