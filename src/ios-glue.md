# iOS Glue

- [Library Target](#library-target)
- [Dependencies](#dependencies)
- [iOS Glue Setup](#ios-glue-setup)
  - [Build File](#build-file)
  - [Errors](#errors)
  - [Prelude](#prelude)
  - [FFI Helpers](#ffi-helpers)
  - [Glue](#glue)
- [Building the Project](#building-the-project)
- [Using the Library](#using-the-library)
  - [Adding XCFramework](#adding-xcframework)
  - [Using XCFramework](#using-xcframework)

Most of our work is going to be on `glue/ios`

## Library Target

Inside `glue/ios/Cargo.toml` add the library target and make the crate type a static library

```toml {hl_lines=["6-8"],linenostart=1}
[package]
name = "ios"
version = "0.1.0"
edition = "2021"
[lib]
name = "exa"
crate-type = ["staticlib"]
```

## Dependencies

- `exa_core`; our core library
- `libc`; provides all of the definitions necessary to easily interoperate with C code (or "C-like" code) on each of the platforms that Rust supports
- `cbindgen` as build dependency; to generate our library header file.

We want to export our library as C interfaces so that we can use C types and C functions directly from Swift. [C interoperability with Swift](https://developer.apple.com/documentation/swift/c-interoperability).

Run

```shell
# From the project root directory
cargo add -p ios --path ./exa_core/
cargo add -p ios libc
cargo add -p ios --build cbindgen
```

## iOS Glue Setup

### Build File

Create `glue/ios/build.rs` and add to our bindings setup

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

Whenever we have a non-mangeled (has `#[no_mangle]` attribute), public, exported C function in our `lib.rs` or declared as a module there, the fuction signature is going to be added to `glue/ios/include/exa_native.h` on save.

Create `glue/ios/include/module.modulemap` and add the proper info to export the headers

```modulemap
module Exa {
    header "exa_native.h"
    export *
}
```

### Errors

Create `glue/ios/src/error.rs` that will contain our errors enum to be handled later on.

```rust
#[derive(Debug)]
pub enum Error {
    InvalidUtf8,
    InvalidUint,
}
```

### Prelude

Prelude is going to be imported in most of the files, so we want to make it as minimal as possibile.

Inside of it we're going to make a shortcut for the `Result` type.

```rust
pub use crate::error::Error;
pub type Result<T> = core::result::Result<T, Error>;
```

Now we can add both of them to `glue/ios/src/lib.rs`

```rust
mod error;
mod prelude;
```

This pattern is considered an idiomatic way in rust for handling errors.

### FFI Helpers

FFI stands for foriegn function interface, we will add helpers for converting to and from C types

Create `glue/ios/src/ffi_helpers.rs`

```rust
use crate::prelude::*;
use libc::c_char;
use std::ffi::{CStr, CString};
pub unsafe fn to_rust_str<'a>(raw_ptr: *const c_char) -> Result<&'a str> {
    match CStr::from_ptr(raw_ptr).to_str() {
        Ok(res) => Ok(res),
        Err(_) => Err(Error::InvalidUtf8),
    }
}
pub fn to_c_str(s: &str) -> Result<*mut i8> {
    match CString::new(s) {
        Ok(string) => Ok(string.into_raw()),
        Err(_) => Err(Error::InvalidUint),
    }
}
```

And finally import `ffi_helpers.rs` inside `lib.rs`

```rust
mod error;
mod prelude;
mod ffi_helpers;
```

### Glue

For the gluing part, we want to import `libc` and `exa_core` and with the help of `ffi_helpers` we can convert back and forth between C and Rust types.

```rust {hl_lines=["5-19"],linenostart=1}
mod error;
mod ffi_helpers;
mod prelude;
use exa_core::greet;
use libc::c_char;
#[no_mangle]
pub unsafe extern "C" fn greet_person(name: *const c_char) -> *mut c_char {
    let Ok(name) = ffi_helpers::to_rust_str(name) else {
        panic!("Failed to convert to rust string slice");
    };
    let name = greet(name);
    let Ok(name) = ffi_helpers::to_c_str(&name) else {
        panic!("Failed to convert to char array");
    };
    name
}
```

The `#[no_mangle]` is an important part, so on build the function name is still the same.

Now if you save, you should be able to see the `exa_native.h` has added a function.

```c
#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>
char *greet_person(const char *name);
```

## Building the Project

Add these commands to the `Makefile`

```make {hl_lines=["4", "18-41"],linenostart=1}
ios_targets = aarch64-apple-ios x86_64-apple-ios aarch64-apple-ios-sim
android_targets = armv7-linux-androideabi i686-linux-android aarch64-linux-android x86_64-linux-android
lib_name = exa
framework_name = Exa
android-setup:
 @for target in ${android_targets} ; do \
  rustup target add $$target ; \
 done
ios-setup:
 cargo install cargo-lipo
 @for target in ${ios_targets} ; do \
  rustup target add $$target ; \
 done
ios: ios-clean ios-build ios-framework
ios-build:
 @echo "Building for iOS..."
 @for target in ${ios_targets} ; do \
  cargo build -p ios --release --target $$target ; \
 done
 @lipo -create \
  target/x86_64-apple-ios/release/lib${lib_name}.a \
  target/aarch64-apple-ios-sim/release/lib${lib_name}.a \
  -output target/lib${lib_name}_sim.a
ios-framework:
 @xcodebuild -create-xcframework \
  -library target/lib${lib_name}_sim.a \
  -headers glue/ios/include/ \
  -library target/aarch64-apple-ios/release/lib${lib_name}.a \
  -headers glue/ios/include/ \
  -output target/${framework_name}.xcframework
ios-clean:
 @cd target && rm -rf ${framework_name}.xcframework
```

_**Note:** Please make sure to use tabs rather than spaces, if we use spaces Makefile will throw this error: `*** missing separator.  Stop.`_

And now run `make ios`

`ios-build` is going to loop over the ios targets that we specified, and build the project targeting them. Then using `lipo` we're going to create `.a` files

And finally we want to bundle the `.a` files inside an `xcframework` so we can use it in our ios app.

## Using the Library

### Adding XCFramework

Navigate to `Frameworks, Libraries, and Embedded Content`

![1]

Choose `Add Other...` > `Add Files...`

![2]

Navigate to `Exa.xcframework` (under `target/`) and Click open

![3]

### Using XCFramework

We're going to import `Exa` and use `greet_person`, simple enough.

```swift
import SwiftUI
import Exa
struct ContentView: View {
    var body: some View {
        VStack {
            Button(action: {
                Exa.greet_person("Ghamza").map {
                    print(String(cString: $0))
                }
            }) {
                Text("Greet")
            }
        }
        .padding()
    }
}
struct ContentView_Previews: PreviewProvider {
    static var previews: some View {
        ContentView()
    }
}
```

After using `Exa.greet_person` we're going to receive a C char pointer, so instead of unsafly unwrapping we use `.map`, if we have a valid result than we can unwrap it and print it on the console.

Finally, after pressing `Greet` you should be able to see the log message in xcode's output console.

---

> You can find the code for this article on GitHub [Ghamza-Jd/exa-lib][4]

[1]: https://static.ghamza.dev/images/rust-for-mobile-part-2/navigate_to_framework.png
[2]: https://static.ghamza.dev/images/rust-for-mobile-part-2/add_other_framework.png
[3]: https://static.ghamza.dev/images/rust-for-mobile-part-2/choose_framework.png
[4]: https://github.com/Ghamza-Jd/exa-lib/tree/part-2-ios-glue
