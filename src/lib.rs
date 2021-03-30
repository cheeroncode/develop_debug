/*!
## Print debugging information at development time.
``` rust
use develop_debug::*;

#[test]
fn use_develop_debug() {
    let x = "dear X";
    let say = "hello world!";
    let array = vec!["a", "b", "c"];
    let title2 = "balabala...";

    develop_debug!(title "example",title2);
    develop_debug!(var x,say);
    develop_debug!(iter array.iter());
    develop_debug!(done "genius!");
    develop_debug!(error "dude, this road is blocked.");
    develop_debug!(
        "{}",
        "Use it just as you would with the `println!()` macro."
    );
}

#[test]
fn use_develop_debug_shortcut() {
    let x = "dear X";
    let say = "hello world!";
    let array = vec!["a", "b", "c"];
    let title2 = "balabala...";

    dd___title!("example", title2);
    dd_____var!(x, say);
    dd____iter!(array.iter());
    dd____done!("genius!");
    dd___error!("dude, this road is blocked.");
    dd________!(
        "{}",
        "Use it just as you would with the `println!()` macro."
    );
}
```
*/

#[cfg_attr(not(feature = "debug"), path = "ignore_debug.rs")]
#[cfg_attr(feature = "debug", path = "enable_debug.rs")]
#[macro_use]
mod debug;

pub use develop_debug as dd________;

/**
## Shortcut to `develop_debug!(done expr)`
*/
#[cfg_attr(
    not(feature = "debug"),
    doc = "The [`develop_debug!`] has been `ignored`."
)]
#[cfg_attr(feature = "debug", doc = "The [`develop_debug!`] has been `enabled`.")]
/**
Configuration `Cargo.toml`, switch debugging;
``` toml
    [dependencies.develop_debug]
    path = "../develop_debug"
    features = ["debug"] # Control switch debugging.
```
debug **`enable`** use `features = ["debug"]`;

debug **`ignore`** use `features = []` or delete this line;
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
#[cfg_attr(
    not(feature = "debug"),
    doc = "The [`develop_debug!`] has been `ignored`."
)]
#[cfg_attr(feature = "debug", doc = "The [`develop_debug!`] has been `enabled`.")]
/**
Configuration `Cargo.toml`, switch debugging;
``` toml
    [dependencies.develop_debug]
    path = "../develop_debug"
    features = ["debug"] # Control switch debugging.
```
debug **`enable`** use `features = ["debug"]`;

debug **`ignore`** use `features = []` or delete this line;
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
#[cfg_attr(
    not(feature = "debug"),
    doc = "The [`develop_debug!`] has been `ignored`."
)]
#[cfg_attr(feature = "debug", doc = "The [`develop_debug!`] has been `enabled`.")]
/**
Configuration `Cargo.toml`, switch debugging;
``` toml
    [dependencies.develop_debug]
    path = "../develop_debug"
    features = ["debug"] # Control switch debugging.
```
debug **`enable`** use `features = ["debug"]`;

debug **`ignore`** use `features = []` or delete this line;
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
#[cfg_attr(
    not(feature = "debug"),
    doc = "The [`develop_debug!`] has been `ignored`."
)]
#[cfg_attr(feature = "debug", doc = "The [`develop_debug!`] has been `enabled`.")]
/**
Configuration `Cargo.toml`, switch debugging;
``` toml
    [dependencies.develop_debug]
    path = "../develop_debug"
    features = ["debug"] # Control switch debugging.
```
debug **`enable`** use `features = ["debug"]`;

debug **`ignore`** use `features = []` or delete this line;
*/
#[macro_export]
macro_rules! dd____iter {
    ($($list:expr),*) => {
        develop_debug!(iter $($list),*);
    };
}

/**
## Shortcut to `develop_debug!(iter expr)`
*/
#[cfg_attr(
    not(feature = "debug"),
    doc = "The [`develop_debug!`] has been `ignored`."
)]
#[cfg_attr(feature = "debug", doc = "The [`develop_debug!`] has been `enabled`.")]
/**
Configuration `Cargo.toml`, switch debugging;
``` toml
    [dependencies.develop_debug]
    path = "../develop_debug"
    features = ["debug"] # Control switch debugging.
```
debug **`enable`** use `features = ["debug"]`;

debug **`ignore`** use `features = []` or delete this line;
*/
#[macro_export]
macro_rules! dd___title {
    ($($msg:expr),*) => {
        develop_debug!(title $($msg),*)
    };
}
