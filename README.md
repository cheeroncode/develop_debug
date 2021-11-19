# Print debugging information at development time

## What does this library do?

* Print nice debugging information;
* Does not affect performance in the release version;

## **Usage**

Add this to your `Cargo.toml` :

``` toml
[dependencies]
develop_debug = "0.6.1"
```

Use the `develop_debug!` :

``` rust
use std::collections::HashMap;

use develop_debug::*;

// Standard usage
#[test]
fn test_develop_debug() {
    let x = "dear X";
    let say = "hello world!";
    let vec = vec!["a", "b", "c"];
    let map = HashMap::from([("a", (Some("a"), "a")), ("b", (Some("b"), "b"))]);
    let title2 = "balabala...";

    develop_debug!(output method);
    develop_debug!(title "example {}",title2);
    develop_debug!(step "do something...{}", say);
    develop_debug!(vars x,say,vec,map);
    develop_debug!(done "genius {}",x);
    develop_debug!(error "dude, this road is blocked. {}",x);
    develop_debug!(
        "{}",
        "Use it just as you would with the `println!()` macro."
    );
}

// Using the shortcut, print the same result as above.
#[test]
fn test_develop_debug_shortcut() {
    let x = "dear X";
    let say = "hello world!";
    let vec = vec!["a", "b", "c"];
    let map = HashMap::from([("a", (Some("a"), "a")), ("b", (Some("b"), "b"))]);
    let title2 = "balabala...";

    dd____show!();
    dd___title!("example {}", title2);
    dd____step!("do something...{}", say);
    dd____vars!(x, say, vec, map);
    dd____done!("genius {}", x);
    dd___error!("dude, this road is blocked. {}", x);
    dd________!(
        "{}",
        "Use it just as you would with the `println!()` macro."
    );
}

#[test]
fn test_output_range_control() {
    dd____show!(); // Output only messages for the current method
    dd____step!("current method 1 .."); // output
    other(); // ignored
    dd____show!(global); // Outputs all messages for all methods
    dd____step!("current method 2 .."); // output
    other(); // output
    dd____hide!(global);
}

fn other() {
    dd____step!("other method");     
}

```

Output in debug mode :

``` sh
❲ tests/how_to_use.rs:36 ❳  🍀  example balabala...

❲ tests/how_to_use.rs:37 ❳  🦀  do something...hello world!

❲ tests/how_to_use.rs:38 ❳  🔹  ‹ x          › = "dear X"
❲ tests/how_to_use.rs:38 ❳  🔹  ‹ say        › = "hello world!"
❲ tests/how_to_use.rs:38 ❳  🔹  ‹ vec        › = [
                                                 ›    "a",
                                                 ›    "b",
                                                 ›    "c",
                                                 ]
❲ tests/how_to_use.rs:38 ❳  🔹  ‹ map        › = {
                                                 ›    "b": (
                                                 ›        Some(
                                                 ›            "b",
                                                 ›        ),
                                                 ›        "b",
                                                 ›    ),
                                                 ›    "a": (
                                                 ›        Some(
                                                 ›            "a",
                                                 ›        ),
                                                 ›        "a",
                                                 ›    ),
                                                 }

❲ tests/how_to_use.rs:39 ❳  🌱  done. genius dear X


❲ tests/how_to_use.rs:40 ❳  💥  error. dude, this road is blocked. dear X


❲ tests/how_to_use.rs:41 ❳  🐰  Use it just as you would with the `println!()` macro.

# output_range_control

🦀  current method 1 ..

🦀  current method 2 ..

🦀  other method

```

No output in `--release` mode.

## **What are the benefits of `develop_debug!` shortcut?**

😬 I think it's easy to recognize in the source code ;  
🤤 I think it's convenient to prompt in `VS Code` ;  
🤓 I think it visually splits up the code ;  

## **Does the `develop_debug!` macro affect code performance?**

* It doesn't affect code performance at all;
* When compiled to the release, `develop_debug!` actually expanded to empty;

## If the document is not semantically fluent

🥺, please forgive my lack of English.
All the documents are explained by the translation software;
If you can provide a better translation, please contact me at [code@autodo.xyz](mailto:code@autodo.xyz);

**I hope this simple library is of some help to you.**

😌 😌 😌 😌 😌 😌  
