use develop_debug::*;

// Standard usage
#[test]
fn use_develop_debug() {
    let x = "dear X";
    let say = "hello world!";
    let array = vec!["a", "b", "c"];
    let title2 = "balabala...";

    develop_debug!(title "example",title2);
    develop_debug!(step "do something...");
    develop_debug!(var x,say);
    develop_debug!(iter array.iter());
    develop_debug!(done "genius!");
    develop_debug!(error "dude, this road is blocked.");
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

    dd___title!("example", title2);
    dd____step!("do something...");
    dd_____var!(x, say);
    dd____iter!(array.iter());
    dd____done!("genius!");
    dd___error!("dude, this road is blocked.");
    dd________!(
        "{}",
        "Use it just as you would with the `println!()` macro."
    );
}
