/**
## The [`develop_debug!`] has been `ignored`.
*/
#[cfg_attr(
    not(feature = "debug"),
    doc = "The [`develop_debug!`] has been `ignored`."
)]
#[cfg_attr(feature = "debug", doc = "The [`develop_debug!`] has been `enabled`.")]
/**
Configuration `Cargo.toml`, switch debugging;
``` toml
    [dependencies.develop_debug]
    path = "../develop_debug"
    features = ["debug"] # Control switch debugging.
```
debug **`enable`** use `features = ["debug"]`;

debug **`ignore`** use `features = []` or delete this line;
*/
#[macro_export]
macro_rules! develop_debug {
    ($($token:tt)*) => {};
}