# Brainrust
### A Brainf*ck interpreter written in rust

## Installation
Easiest way is to install from crates.io.
```
cargo install brainrust
```
Alternatively, you can install from github.
```
git clone https://github.com/EthanYidong/brainrust
cd brainrust
cargo install --path .
```

## Usage
Run a brainf*ck file:
```
bfrust <file>
```
Compile to a binary file for execution by translating to rust and compiling using rustc:
```
bfrust <file> -c <out>
```
