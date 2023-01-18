# Setup Makefile

In this section we are going to create `Makefile` that will contain our commands.

```bash
touch Makefile
```

We will start by adding android and iOS targets

```Makefile
ios_targets = aarch64-apple-ios \
	x86_64-apple-ios \
	aarch64-apple-ios-sim

android_targets = armv7-linux-androideabi \
	i686-linux-android \
	aarch64-linux-android \
	x86_64-linux-android
```

Now we can loop over those targets to do whatever we want, as a beginning we
can add this targets to our tool chain under a setup command.

```Makefile [hl,10-18]
ios_targets = aarch64-apple-ios \
	x86_64-apple-ios \
	aarch64-apple-ios-sim

android_targets = armv7-linux-androideabi \
	i686-linux-android \
	aarch64-linux-android \
	x86_64-linux-android

android-setup:
	@for target in ${android_targets} ; do \
        rustup target add $$target ; \
    done

ios-setup:
	@for target in ${ios_targets} ; do \
        rustup target add $$target ; \
    done
```

For iOS we also want to install `cargo-lipo`, which automatically creates a
universal library for use with your iOS application.

```Makefile [hl,7]
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

Now to set our iOS and android from rust side we run simple

```bash
make ios-setup
make android-setup
```

> **📄 Note:** *We should use hard tabs in our `Makefile`, spaces will result in
> "missing separator"*

To make sure we use hard tabs we can add this rule in our `.editorconfig`.

```toml [hl,9-11]
# https://editorconfig.org

root = true

[*]
insert_final_newline = true
trim_trailing_whitespace = true

[Makefile]
indent_style = tab
indent_size = 4
```
