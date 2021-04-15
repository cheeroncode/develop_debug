/*!
## Print debugging information at development time.

* Print nice debugging information;
* Does not affect performance in the release version;

[See the manual](https://crates.io/crates/develop_debug)
*/

mod copy_mode;

pub use develop_debug as dd________;

/**
## Shortcut to `develop_debug!(done fmt,expr)`
*/
#[macro_export]
macro_rules! dd____done {
    ($fmt:literal $(,)? $($msg:expr),*) => {
        develop_debug!(done $fmt $($msg),*);
    };
}

/**
## Shortcut to `develop_debug!(error fmt,expr)`
*/
#[macro_export]
macro_rules! dd___error {
    ($fmt:literal $(,)? $($msg:expr),*) => {
        develop_debug!(error $fmt $($msg),*);
    };
}

/**
## Shortcut to `develop_debug!(var expr)`
*/
#[macro_export]
macro_rules! dd_____var {
    ($($arg:expr),*) => {
        develop_debug!(var $($arg),*);
    };
}

/**
## Shortcut to `develop_debug!(iter expr)`
*/
#[macro_export]
macro_rules! dd____iter {
    ($($arg:expr),*) => {
        develop_debug!(iter $($arg),*);
    };
}

/**
## Shortcut to `develop_debug!(step expr)`
*/
#[macro_export]
macro_rules! dd____step {
    ($fmt:literal $(,)? $($msg:expr),*) => {
        develop_debug!(step $fmt $($msg),*);
    };
}

/**
## Shortcut to `develop_debug!(title expr)`
*/
#[macro_export]
macro_rules! dd___title {
    ($fmt:literal $(,)? $($msg:expr),*) => {
        develop_debug!(title $fmt $($msg),*);
    };
}
