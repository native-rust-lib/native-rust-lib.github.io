# Rust 🦀 on Mobile 📱

This small book describes how to use Rust and Mobile (android and iOS) together.

## Who is this book for?

This book is for anyone interested in compiling Rust to android or iOS for fast,
reliable code on the mobile. You should know some Rust, and be familiar with
Kotlin and Swift. You don't need to be an expert in any of them.

## How to read this book?

The book is written to be read from start to finish. You should follow
along:writing, compiling, and running the tutorial's code yourself.

We will build a rust library in this series and port it to different platforms.
We are calling it `libexa` (which stands for Example Library 🫣).

## Low-Level Control with High-Level Ergonomics

Rust gives programmers low-level control and reliable performance. It is free
from the non-deterministic garbage collection pauses that plague Kotlin/Swift.
Programmers have control over indirection, monomorphization, and memory layout.

## Do _Not_ Rewrite Everything

Existing code bases don't need to be thrown away. You can start by porting your
most performance-sensitive Swift/Kotlin functions to Rust to gain immediate
benefits. And you can even stop there if you want to.

I strongly recommend [installing all of your Rust toolchains using rustup.][1]

> _The code of this tutorial is available on [Ghamza-Jd/exa-lib][2]_

[1]: https://www.rust-lang.org/tools/install
[2]: https://github.com/Ghamza-Jd/exa-lib
