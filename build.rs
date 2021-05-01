use std::fs;
fn main() {
    // If in the `docs.rs` server sandbox, skip the build script.
    if let Ok(_) = std::env::var("DOCS_RS") {        
        return;
    }

    println!("cargo:rerun-if-changed=Cargo.toml");
    println!("cargo:rerun-if-changed=T.README.md");
    println!("cargo:rerun-if-changed=/src/lib.rs");
    // Use the file that corresponds to the compilation option
    // if cfg!(debug_assertions) {
    //     // println!("cargo:warning=The crate 'develop_debug' is running in debug mode");
    //     println!("cargo:rustc-env=develop_debug=true");
    //     let enable = fs::copy("./src/mode/debug_mode.rs", "./src/copy_mode.rs");
    //     if let Err(err) = enable {
    //         println!("cargo:warning=Failed to copy file 'debug_mode.rs'. {}", err);
    //     }
    // } else {
    //     println!("cargo:rustc-env=develop_debug=false");
    //     // println!("cargo:warning=The crate 'develop_debug' is running in release mode");
    //     let ignore = fs::copy("./src/mode/release_mode.rs", "./src/copy_mode.rs");
    //     if let Err(err) = ignore {
    //         println!(
    //             "cargo:warning=Failed to copy file 'release_mode.rs'. {}",
    //             err
    //         );
    //     }
    // }
    // If the version changes, change the version in the file `copy_mode.rs`.
    // let copy = fs::read_to_string("./src/copy_mode.rs");
    // if let Ok(s) = copy {
    //     let update_ver = s.replace(
    //         "version = \"...\"",
    //         &format!("version = \"{}\"", env!("CARGO_PKG_VERSION")),
    //     );
    //     if let Err(err) = fs::write("./src/copy_mode.rs", update_ver) {
    //         println!(
    //             "cargo:warning=Failed to update version in './src/copy_mode.rs'. {}",
    //             err
    //         );
    //     }
    // }
    // If the version changes, change the version in the file `README.md`.
    let tr = fs::read_to_string("./T.README.md");
    if let Ok(s) = tr {
        let t = s.replace("{ver}", env!("CARGO_PKG_VERSION"));
        let how = fs::read_to_string("./tests/how_to_use.rs");
        if let Ok(h) = how {
            let readme = t.replace("{how_to_use}", &h);
            if let Err(err) = fs::write("./README.md", readme) {
                println!("cargo:warning=Failed to write './README.md'. {}", err);
            }
        }
    }
}
