# roblox-rs

DISCLAIMER: The transpiler, if built from source, has an EXTREMELY big chance to break. This is
due to the fact that `roblox-rs` uses internal rust crates, which are very unstable and are
constantly changing.

Why use internal crates instead of something like [syn](https://crates.io/crates/syn)? Well,
`syn` doesn't support stuff like lifetime resolution and macro expanding, so I opted into
using internal crates.

## building

To build `roblox-rs`, first acquire a nightly version of the Rust compiler with [rustup](https://rustup.rs/).

If you are installing Rust for the first time, you can select the nightly version within `rustup`,
if you aren't however, you can run the following:

```console
$ rustup toolchain default nightly
```

Afterwards, you need to install a few required components:

```console
$ rustup component add rustc-dev rust-src llvm-tools-preview
```

And finally, you should clone the repository, and build it using:

```console
$ cargo build
```

## usage

Currently, `roblox-rs` crashes when it encounters any type of AST node. Wow! What a release!
<!-- Currently, `roblox-rs` cannot be used within a Rojo project, but it can transpile standalone `.rs` files. -->