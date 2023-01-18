# Setup 🏗️

## The Rust Toolchain

You will need the standard Rust toolchain, including `rustup`, `rustc`, and `cargo`.

[Follow these instructions to install the Rust toolchain.][1]

## Setting up the project

Let's get started and create our project. We will not use `cargo new --lib` yet;
instead, we will make an empty directory and initialize git.

```bash
mkdir exa-lib
cd exa-lib
git init
```

> **📄 Note:** *We initialized git that early, so when we create a new rust library,
> it doesn’t come with a version control system; if you want to create a lib
> without `vcs`, add `--vcs none` to your `cargo new` command.*

Each rust project (yes, we will create multiple rust projects) will be a member
of our [cargo's workspace.][2]

```bash
touch Cargo.toml
```

We can now build our core library, iOS and Android glues.

```bash
cargo new --lib exa_core
cargo new --lib glue/android
cargo new --lib glue/ios
```

The rust analyzer will go mad at us, so we should add them to our workspace.

```toml
[workspace]
members = [
    "exa_core",
    "glue/android",
    "glue/ios"
]
```

And add one of the most essential files to git, `.gitignore`

```text
debug/
target/
Cargo.lock

**/*.rs.bk

*.pdb

.DS_Store

.idea/
.fleet/
.vscode/

main.rs
```

We are ignoring the `.lock` file since we’re building a library. And I also
like to ignore `main.rs` when building a library, so it acts as my playground
for drafting rust code, and indeed we don’t want that to be saved in the repo.

And for our last file for this section, the `.editorconfig`

```toml
# https://editorconfig.org

root = true

[*]
insert_final_newline = true
trim_trailing_whitespace = true
```

> **💡 Tip:** *Install the [EditorConfig][3] extension
> on your text editor if it doesn't support it out of the box.*

And we’re ending this chapter by adding a small function in `src/exa_core/lib.rs`
to greet someone.

```rust
pub fn greet(person: &str) -> String {
    format!("Hello, {person}")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_greets() {
        let result = greet("World");
        assert_eq!(result, "Hello, World");
    }
}
```

[1]: https://www.rust-lang.org/tools/install
[2]: https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html
[3]: https://editorconfig.org/
