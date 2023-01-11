# Introduction

In this series, we will create a rust library and port it to different platforms, couldn't come up with a good name so I we're going to call it `libexa` (as Example Library).

<!-- To get the most out of this series you should be using macOS to build for iOS. If you're on windows or linux you can't build for iOS. -->

To build for android and iOS, you should be on macOS. If you're using windows or linux, you can only build for android, but worth noting that some commands and setups might be different (for example, you might not have `make`, `python`, or `brew` on your machine).

You can use the development environment you're most comfortable with, but mine is [VSCode](https://code.visualstudio.com/) with these extensions:

- [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
- [Better TOML](https://marketplace.visualstudio.com/items?itemName=bungcip.better-toml)
- [CodeLLDB](https://marketplace.visualstudio.com/items?itemName=vadimcn.vscode-lldb)

I also use [Fig](https://fig.io/) to get suggestions on the terminal.
