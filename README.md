DangerousCode
=============

A rust-lang attribute meant to mark code as dangerous

How to use
----------
Add the following to your Cargo.toml
```
[dependencies.dangerouscode]
git = "https://github.com/6720/dangerouscode-rs"
```
Include the `dangerouscode` crate.
```
#![feature(plugin)]
#![plugin(dangerouscode)]

extern crate dangerouscode;
```
(If there's a better way to do this, please let me know!)

Use the `#[dangerous]` attribute on something dangerous... 
```rust
#[dangerous]
fn main() {
        println!("I'm dangerous! rawr!");
}
```
...And there will be a compiler warning!
```
src/main.rs:6:1: 6:13 warning: Item 'main' is dangerous!
src/main.rs:6 #[dangerous]
```
