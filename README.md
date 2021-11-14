# Print debugging information at development time

## What does this library do?

* Print nice debugging information;
* Does not affect performance in the release version;

## **Usage**

Add this to your `Cargo.toml` :

``` toml
[dependencies]
develop_debug = "0.5.0"
```

Use the `develop_debug!` :

``` rust
use develop_debug::*;

// Standard usage
#[test]
fn use_develop_debug() {
    let x = "dear X";
    let say = "hello world!";
    let array = vec!["a", "b", "c"];
    let title2 = "balabala...";

    develop_debug!(output method);
    develop_debug!(title "example {}",title2);
    develop_debug!(step "do something...{}", say);
    develop_debug!(var x,say);
    develop_debug!(iter array.iter(),array);
    develop_debug!(done "genius {}",x);
    develop_debug!(error "dude, this road is blocked. {}",x);
    develop_debug!(
        "{}",
        "Use it just as you would with the `println!()` macro."
    );
}

// Using the shortcut, print the same result as above.
#[test]
fn use_develop_debug_shortcut() {
    let x = "dear X";
    let say = "hello world!";
    let array = vec!["a", "b", "c"];
    let title2 = "balabala...";

    dd____show!();
    dd___title!("example {}", title2);
    dd____step!("do something...{}", say);
    dd_____var!(x, say);
    dd____iter!(array.iter(), array);
    dd____done!("genius {}", x);
    dd___error!("dude, this road is blocked. {}", x);
    dd________!(
        "{}",
        "Use it just as you would with the `println!()` macro."
    );
}

#[test]
fn use_output_range_control() {    
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
🍀  example balabala...

🦀  do something...hello world!

🔹  ‹ x          › = ‹"dear X"›
🔹  ‹ say        › = ‹"hello world!"›

🔶  array.iter()
🔸  "a"
🔸  "b"
🔸  "c"

🔶  array
🔸  "a"
🔸  "b"
🔸  "c"


🌱  done. genius dear X


💥  error. dude, this road is blocked. dear X

🐰  Use it just as you would with the `println!()` macro.

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
