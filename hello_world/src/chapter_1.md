# Chapter 1  Core Concepts and Borrowing 

## The Main Function

All rust starts in the main function.  

```rust,editable
fn main(){
    println!("Hello WOrld");
}
```

---

## The Rust Compiler

```
rustc main.rs
./main
```

 The vast majority of Rust projects use Cargo....which is a build system and package manager in-one.

 ## Cargo

For cargo you neeed a simple config file like this:

```toml
[package]
name = "hello_cargo"
version = "0.1.0"
edition = "2024"

[dependencies]
```


