# Setup

- [Creating the project](#creating-the-project)
- [Creating Rust Libraries](#creating-rust-libraries)
- [Makefile](#makefile)
- [Install Android and iOS Targets](#install-android-and-ios-targets)
- [Add Minimal Code Inside Core](#add-minimal-code-inside-core)

## Creating the project

1. We're not going to use `cargo new --lib` yet; instead, we're going to create an empty directory.

    ```shell
    mkdir exa-lib
    cd exa-lib
    ```

2. Then inside of it, we want `Cargo.toml`, `.gitignore`, `Makefile`, and `glue/`.

    ```shell
    touch Cargo.toml .gitignore Makefile
    mkdir glue
    ```

    `Cargo.toml` will contain our rust workspace members, and inside `Makefile` we will put our commands.

3. Add ignore patterns

    ```txt
    debug/
    target/
    **/*.rs.bk
    *.pdb
    .DS_Store
    .idea/
    .fleet/
    .vscode/
    main.rs
    ```

    I usually ignore `main.rs` for libs so I can keep it locally as a playground without pushing my draft code on the repo

4. And finally, initialize git.

    ```shell
    git init
    ```

## Creating Rust Libraries

Now we're going to create three rust libraries

```shell
cargo new --lib exa_core
cargo new --lib glue/android
cargo new --lib glue/ios
```

Then the rust analyzer will go mad, so let us add them to the workspace inside the `Cargo.toml` on the root directory.

```toml
[workspace]
members = [
    "exa_core",
    "glue/android",
    "glue/ios"
]
```

## Makefile

Inside it, we're going to add some commands to make our life easier for the setup and building.

```make
ios_targets = aarch64-apple-ios x86_64-apple-ios aarch64-apple-ios-sim
android_targets = armv7-linux-androideabi i686-linux-android aarch64-linux-android x86_64-linux-android
lib_name = exa
android-setup:
	@for target in ${android_targets} ; do \
		rustup target add $$target ; \
	done
ios-setup:
	cargo install cargo-lipo
	@for target in ${ios_targets} ; do \
		rustup target add $$target ; \
	done
```

_**Note:** Please make sure to use tabs rather than spaces, if we use spaces Makefile will throw this error: `Makefile:6: *** missing separator.  Stop.`_

In other parts of this series, we will add more commands to this file.

## Install Android and iOS Targets

In the previous step, we added commands to setup both Android and iOS, so now let us run them

```shell
make android-setup
make ios-setup
```

## Add Minimal Code Inside Core

Inside `exa_core/src/lib.rs` replace the file's content with a simple greeting function

```rust
pub fn greet(person: &str) -> String {
    format!("Hello {person}")
}
```

---

> You can find the code for this article on GitHub [Ghamza-Jd/exa-lib][1]

[1]: https://github.com/Ghamza-Jd/exa-lib/tree/part-1-setup
