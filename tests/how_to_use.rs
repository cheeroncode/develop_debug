use std::{collections::HashMap, iter::FromIterator};

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
