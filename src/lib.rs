/*!
## Print debugging information at development time.

* Print nice debugging information;
* Does not affect performance in the release version;

[See the manual](https://crates.io/crates/develop_debug)
*/

mod copy_mode;

pub use develop_debug as dd________;

/**
## Shortcut to `develop_debug!(done expr)`
*/
#[macro_export]
macro_rules! dd____done {
    ($($msg:expr),*) => {
        develop_debug!(done $($msg),*);
    };
}

/**
## Shortcut to `develop_debug!(error expr)`
*/
#[macro_export]
macro_rules! dd___error {
    ($($msg:expr),*) => {
        develop_debug!(error $($msg),*);
    };
}

/**
## Shortcut to `develop_debug!(var expr)`
*/
#[macro_export]
macro_rules! dd_____var {
    ($($var:expr),*) => {
        develop_debug!(var $($var),*);
    };
}

/**
## Shortcut to `develop_debug!(iter expr)`
*/
#[macro_export]
macro_rules! dd____iter {
    ($($list:expr),*) => {
        develop_debug!(iter $($list),*);
    };
}

/**
## Shortcut to `develop_debug!(step expr)`
*/
#[macro_export]
macro_rules! dd____step {
    ($($msg:expr),*) => {
        develop_debug!(step $($msg),*);
    };
}

/**
## Shortcut to `develop_debug!(title expr)`
*/
#[macro_export]
macro_rules! dd___title {
    ($($msg:expr),*) => {
        develop_debug!(title $($msg),*);
    };
}
