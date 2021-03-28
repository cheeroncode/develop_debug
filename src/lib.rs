/**
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
#[macro_export]
#[cfg(not(feature = "debug"))]
macro_rules! develop_debug {
    ($($token:tt)*) => {};
}

/**
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
#[macro_export]
#[cfg(feature = "debug")]
macro_rules! develop_debug {

    (title $($msg:expr),*) => {
        $(
            println!("🍀  {}",$msg);
        )*
    };

    (var $($arg:expr),*)=>{
        $(
            println!("🔹  ‹ {:^5} › = ‹{}›",stringify!($arg),$arg);
        )*
    };

    (iter $($list:expr),*)=>{
        $(
            println!("🔶  {}",stringify!($list));
            for item in $list{
                println!("🔸  {:?}",item);
            }
        )*
    };

    (done $($msg:expr),*) => {
        println!("🌱  done.");
        $(
            println!("🌱  {}",$msg);
        )*
        println!();
    };

    (error $($msg:expr),*) => {
        eprintln!("💥  error.");
        $(
            eprintln!("💥  {}",$msg);
        )*
        println!();
    };
    ($($args:tt)*) => {
        print!("🐰  ");
        println!($($args)*);
    };
}

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
## Shortcut to `develop_debug!(iter expr)`
*/
#[macro_export]
macro_rules! dd___title {
    ($($msg:expr),*) => {
        develop_debug!(title $($msg),*)
    };
}
