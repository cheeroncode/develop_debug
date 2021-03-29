# Print debugging information at development time

## What does this library do?

* Print nice debugging information;
* Does not affect performance in the release version;

## **Usage**

Add this to your `Cargo.toml` :

``` toml
[dependencies]
develop_debug = { version = "0.1.1", features = ["debug"] }
```

Use the `develop_debug!` :

``` rust
use develop_debug::*;

let x = "dear X";
let say = "hello world!";
let array = vec!["a", "b", "c"];
let title2 = "balabala...";

develop_debug!(title "example",title2);
develop_debug!(var x,say);
develop_debug!(iter array.iter());
develop_debug!(done "genius!");
develop_debug!(error "dude, this road is blocked.");
develop_debug!("{}","Use it just as you would with the `println!()` macro.");
```

Output:

``` output
🍀  example
🍀  balabala...
🔷  ‹   x   › = ‹dear X›
🔷  ‹  say  › = ‹hello world!›
🔶  array.iter()
🔸  "a"
🔸  "b"
🔸  "c"
🌱  done.
🌱  genius!

💥  error.
💥  dude, this road is blocked.

🐰  Use it just as you would with the `println!()` macro.
```

Using the `develop_debug!` shortcut, print the same output as above :

``` rust
use develop_debug::*;

let x = "dear X";
let say = "hello world!";
let array = vec!["a", "b", "c"];
let title2 = "balabala...";

dd___title!("example", title2);
dd_____var!(x, say);
dd____iter!(array.iter());
dd____done!("genius!");
dd___error!("dude, this road is blocked.");
dd________!("{}","Use it just as you would with the `println!()` macro.");
```

## **What are the benefits of `develop_debug!` shortcut?**

😬 I think it's easy to recognize in the source code ;  
🤤 I think it's convenient to prompt in `VS Code` ;  
🤓 I think it visually splits up the code ;  

## **How do development debug macros affect the code in the release?**

😬 Add this to your `Cargo.toml` when developing :

``` toml
[dependencies]
develop_debug = { version = "0.1.1", features = ["debug"] }
```

🤓 Modify `Cargo.toml` when you are ready to release :

``` toml
[dependencies]
develop_debug = { version = "0.1.1" }
```

* There is no need to clear debugging methods from source code;
* The `develop_debug!` macro expands to empty;
* It doesn't affect code performance at all;

## If the document is not semantically fluent

🥺, please forgive my lack of English.
All the documents are explained by the translation software;
If you can provide a better translation, please contact me at [code@autodo.xyz](mailto:code@autodo.xyz);

**I hope this simple library is of some help to you.**

## Some amazing mistakes are not mistakes

After switching features, then execute `cargo` to report an error:

``` sh
cargo build
...
error: failed to remove .../target/debug/deps/generates-...: No such file or directory (os error 2)
# The solution
# Switch features, and then
# Clear old compiled content
# Execute the required command
cargo clean
cargo build
```