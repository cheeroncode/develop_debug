# Print debugging information at development time

## What does this library do?

* Print nice debugging information;
* Does not affect performance in the release version;

## **Usage**

Add this to your `Cargo.toml` :

``` toml
[dependencies]
develop_debug = "{ver}"
```

Use the `develop_debug!` :

``` rust
{how_to_use}
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
