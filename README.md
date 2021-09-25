# Primes-rs

## Description

Primes-rs is a rust binary project, implementing the sieve of Eratosthenes in Rust. Initially it was a single-threaded application, but as a learning exercise I decided to try to multithread it.

## Usage

The program accepts one command-line parameter (`upper-limit`), which as the name suggests sets the upper limit for the program. If this parameter is either not supplied or is not a number, it will panic. Displaying the message: **Invalid number entered**.

## Building & Running

### Debug

Use `cargo b` to build the project. If you intend to run it you can simply use `cargo r -- <upper-limit>`.

### Release

For building in Release mode run `cargo b --release`. The same applies also to running the program `cargo r --release -- <upper-limit>`.
