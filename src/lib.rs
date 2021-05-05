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
pub fn enable_output() -> bool {
    DEVELOP_DEBUG_OUTPUT_STATE.with(|state| *state.borrow())
}

/**
## Print debugging information at development time.
[See the manual](https://crates.io/crates/develop_debug)
*/
#[cfg(debug_assertions)]
#[macro_export]
macro_rules! develop_debug {

    (output true)=>{
        debug(true);
    };

    (output false)=>{
        debug(false);
    };

    (title $(,)? $fmt:literal $(,)? $($msg:expr),*) => {
        if enable_output() {
            println!();
            print!("🍀  ");
            println!($fmt,$($msg),*);
        }
    };

    (step $(,)? $fmt:literal $(,)? $($msg:expr),*) => {
        if enable_output() {
            println!();
            print!("🦀  ");
            println!($fmt,$($msg),*);
        }
    };

    (var $(,)? $($arg:expr),*)=>{
        if enable_output() {
            println!();
            $(
                let dd_var = &$arg;
                let dd_var_name = stringify!($arg);
                println!("🔹  ‹ {:<10} › = ‹{:?}›",dd_var_name,dd_var);
            )*
        }
    };

    (iter $(,)? $($arg:expr),*)=>{
        if enable_output() {
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
        }
    };

    (done $(,)? $fmt:literal $(,)? $($msg:expr),*) => {
        if enable_output() {
            println!();
            print!("🌱  done. ");
            println!($fmt,$($msg)*);
            println!();
        }
    };

    (error $(,)? $fmt:literal $(,)? $($msg:expr),*) => {
        if enable_output() {
            println!();
            print!("💥  error. ");
            println!($fmt,$($msg)*);
            println!();
        }
    };

    ($($args:tt)*) => {
        if enable_output() {
            print!("🐰  ");
            println!($($args)*);
        }
    };
}

pub use develop_debug as dd________;

/**
## Shortcut to `develop_debug!(output true)`
*/
#[macro_export]
macro_rules! dd____show {
    () => {
        develop_debug!(output true);
    };
}

/**
## Shortcut to `develop_debug!(output false)`
*/
#[macro_export]
macro_rules! dd____hide {
    () => {
        develop_debug!(output false);
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
