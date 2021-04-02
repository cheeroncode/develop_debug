/**
## Print debugging information at development time.
[See the manual](https://crates.io/crates/develop_debug)
*/
#[macro_export]
macro_rules! develop_debug {

    (title $($msg:expr),*) => {
        $(
            println!("🍀  {}",$msg);
        )*
    };

    (step $($msg:expr),*) => {
        $(
            println!("🌧  {}",$msg);
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
