# iOS Glue

## Library Target

Make the crate type a static library inside `glue/ios/Cargo.toml`.

```toml [hl,6-8]
[package]
name = "ios"
version = "0.1.0"
edition = "2021"

[lib]
name = "exa"
crate-type = ["staticlib"]
```

Now we want to install our dependencies:

```bash
# From the project root directory
cargo add -p ios --path ./exa_core/
cargo add -p ios libc
cargo add -p ios --build cbindgen
```

We want to export our library as C interfaces so that we can use C types and
C functions directly from Swift (or objective-c). [C interoperability with Swift][1].

> - _`exa_core`; our core library._
> - _`libc`; provides all of the definitions necessary to easily interoperate
> with C code (or "C-like" code) on each of the platforms that Rust supports._
> - _`cbindgen` as build dependency; to generate our library's header file._

First we are going to use `cbindgen` to create a build script that runs on save,
we want our header file to be modified on change rather than manually writing it
down. So create `glue/ios/build.rs`

```rust
extern crate cbindgen;
use std::env;
use cbindgen::Language::C;

fn main() {
    setup_cbindgen();
}

fn setup_cbindgen() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR");
    match crate_dir {
        Ok(val) => {
            cbindgen::Builder::new()
                .with_crate(val)
                .with_language(C)
                .generate()
                .expect("Unable to generate bindings")
                .write_to_file("include/exa_native.h");
        }
        Err(err) => {
            println!("Error: {}", err);
        }
    }
}
```

Now the header file `exa_native.h` is going to live in
`glue/ios/include/exa_native.h`. For exporting it we will create `glue/ios/include/module.modulemap`.

```modulemap
module Exa {
    header "exa_native.h"
    export *
}
```

[1]: https://developer.apple.com/documentation/swift/c-interoperability
