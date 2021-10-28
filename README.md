# RUST-PLAYGROUND
[![jskyzero](https://img.shields.io/badge/Author-jskyzero-brightgreen.svg?style=flat)](/)
[![2020/06/30](https://img.shields.io/badge/Data-2020/06/30-brightgreen.svg?style=flat)](/)
[![Rust](https://github.com/jskyzero/Rust.Playground/actions/workflows/rust.yml/badge.svg)](https://github.com/jskyzero/Rust.Playground/actions/workflows/rust.yml)


## Overview

I decide to learn rust for it's safe memary manage system. maybe I will use rust in Computer Graphics & Game Develop


## Structure

+ assets
  + obj, img, shader, and other assets
+ config
  + proto, config lib
+ engine
  + engine lib
+ leetcode
  + leetcode exserise
+ src
  + main src code
+ tools
  + some build scripts


## Tips

+ install: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
+ offiline docs: `rustup docs`, `rustup docs --book`
+ single file: `rustc helloworld.rs`
+ new project: `cargo new playground`
+ protobuf

```Bash
# https://github.com/stepancheg/rust-protobuf/tree/master/protobuf-codegen
brew install protobuf
cargo install protobuf-codegen
protoc --rust_out src/ proto/*
```
