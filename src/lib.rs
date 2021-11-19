/*!
## Print debugging information at development time.

* Print nice debugging information;
* Does not affect performance in the release version;

[See the manual](https://crates.io/crates/develop_debug)
*/

/**
## Print debugging information at development time.
Run the macro in '-- release' mode and expand the debug message to be empty.
*/
#[cfg(not(debug_assertions))]
#[macro_export]
macro_rules! develop_debug {
    ($($args:tt)*) => {};
}

thread_local! {
    /**
    ## Develop debug global output state (default `false`).

    When the state is `true`, the debug message is printed.
    When the state is `false`, debug messages are ignored.
    */
    pub static DEVELOP_DEBUG_OUTPUT_STATE: std::cell::RefCell<bool> = std::cell::RefCell::new(false);
}

/**
## Sets the global output state of the debug message.
*/
pub fn debug(output: bool) {
    DEVELOP_DEBUG_OUTPUT_STATE.with(|state| *state.borrow_mut() = output);
}

/**
## Gets the global output state of the debug message.
*/
pub fn enable_develop_debug_output() -> bool {
    DEVELOP_DEBUG_OUTPUT_STATE.with(|state| *state.borrow())
}

/**
## Print debugging information at development time.
[See the manual](https://crates.io/crates/develop_debug)
*/
#[cfg(debug_assertions)]
#[macro_export]
macro_rules! develop_debug {
    (output method)=>{
        fn enable_develop_debug_output() -> bool { true }
    };

    (output global true)=>{
        debug(true);
    };

    (output global false)=>{
        debug(false);
    };

    (title $(,)? $fmt:literal $(,)? $($msg:expr),*) => {
        if enable_develop_debug_output() {
            println!();
            print!("❲ {}:{} ❳",file!(),line!());
            print!("  🍀  ");
            println!($fmt,$($msg),*);
        }
    };

    (step $(,)? $fmt:literal $(,)? $($msg:expr),*) => {
        if enable_develop_debug_output() {
            println!();
            print!("❲ {}:{} ❳",file!(),line!());
            print!("  🦀  ");
            println!($fmt,$($msg),*);
        }
    };

    (vars $(,)? $($arg:expr),*)=>{
        if enable_develop_debug_output() {
            println!();
            $(
                let prefix = format!("❲ {}:{} ❳  🔹  ‹ {:<10} › = ",file!(),line!(),stringify!($arg));
                let spaces = " ".repeat(prefix.chars().count());
                let value = format!("{:#?}",&$arg);
                let lines = value.lines().into_iter().map(|s| s).collect::<Vec<&str>>();
                let lines = lines.as_slice();

                if lines.len()==1{
                    for s in lines{
                        println!("{}{}",prefix,s);
                    }
                }else{
                    if let [first,body @ ..,last] = lines{
                        println!("{}{}",prefix,first);
                        for s in body {
                            println!("{} ›{}",spaces,s);
                        }
                        println!("{} {}",spaces,last);
                    }
                }
            )*
        }
    };

    (done $(,)? $fmt:literal $(,)? $($msg:expr),*) => {
        if enable_develop_debug_output() {
            println!();
            print!("❲ {}:{} ❳",file!(),line!());
            print!("  🌱  done. ");
            println!($fmt,$($msg),*);
            println!();
        }
    };

    (error $(,)? $fmt:literal $(,)? $($msg:expr),*) => {
        if enable_develop_debug_output() {
            println!();
            print!("❲ {}:{} ❳",file!(),line!());
            print!("  💥  error. ");
            println!($fmt,$($msg),*);
            println!();
        }
    };

    ($($args:tt)*) => {
        if enable_develop_debug_output() {
            println!();
            print!("❲ {}:{} ❳",file!(),line!());
            print!("  🐰  ");
            println!($($args)*);
            println!();
        }
    };
}

pub use develop_debug as dd________;

/**
## Shortcut to `develop_debug!()`
```
# use develop_debug::*;
dd____show!(); //  develop_debug!(output method);
dd____show!(global); // develop_debug!(output true);
```
*/
#[macro_export]
macro_rules! dd____show {
    () => {
        develop_debug!(output method);
    };
    (global) => {
        develop_debug!(output global true);
    };
}

/**
## Shortcut to `develop_debug!(output false)`
*/
#[macro_export]
macro_rules! dd____hide {
    (global) => {
        develop_debug!(output global false);
    };
}

/**
## Shortcut to `develop_debug!(done fmt,expr)`
*/
#[macro_export]
macro_rules! dd____done {
    () => {
        develop_debug!(done "");
    };
    ($fmt:literal $(,)? $($msg:expr),*) => {
        develop_debug!(done $fmt $($msg),*);
    };
}

/**
## Shortcut to `develop_debug!(error fmt,expr)`
*/
#[macro_export]
macro_rules! dd___error {
    () => {
        develop_debug!(error "");
    };
    ($fmt:literal $(,)? $($msg:expr),*) => {
        develop_debug!(error $fmt $($msg),*);
    };
}

/**
## Shortcut to `develop_debug!(var expr)`
*/
#[macro_export]
macro_rules! dd____vars {
    ($($arg:expr),*) => {
        develop_debug!(vars $($arg),*);
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
