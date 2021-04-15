/**
## Print debugging information at development time.
[See the manual](https://crates.io/crates/develop_debug)
*/
#[macro_export]
macro_rules! develop_debug {

    (title $(,)? $fmt:literal $(,)? $($msg:expr),*) => {
        println!();
        print!("🍀  ");
        println!($fmt,$($msg),*);
    };

    (step $(,)? $fmt:literal $(,)? $($msg:expr),*) => {
        println!();
        print!("🦀  ");
        println!($fmt,$($msg),*);
    };

    (var $(,)? $($arg:expr),*)=>{
        println!();
        $(
            let dd_var = &$arg;
            let dd_var_name = stringify!($arg);
            println!("🔹  ‹ {:<10} › = ‹{:?}›",dd_var_name,dd_var);
        )*
    };

    (iter $(,)? $($arg:expr),*)=>{
        println!();
        $(
            let dd_var = $arg;
            let dd_var_name = stringify!($arg);
            println!("🔶  {}",dd_var_name);
            for item in dd_var{
                println!("🔸  {:?}",item);
            }
            println!();
        )*
    };

    (done $(,)? $fmt:literal $(,)? $($msg:expr),*) => {
        println!();
        print!("🌱  done. ");
        println!($fmt,$($msg)*);        
        println!();
    };

    (error $(,)? $fmt:literal $(,)? $($msg:expr),*) => {
        println!();
        print!("💥  error. ");
        println!($fmt,$($msg)*);
        println!();
    };

    ($($args:tt)*) => {
        print!("🐰  ");
        println!($($args)*);
    };
}
