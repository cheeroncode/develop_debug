/**
## Print debugging information at development time.
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
/**
## Example:
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
        println!("💥  error.");
        $(
            println!("💥  {}",$msg);
        )*
        println!();
    };
    ($($args:tt)*) => {
        print!("🐰  ");
        println!($($args)*);
    };
}
