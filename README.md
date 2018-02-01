dubni
=====

A browser game written in Rust.

Building
--------

To build this project, you will need a nightly compiler
with support for the `wasm32-unknown-unknown`-target.
Using [rustup](https://www.rustup.rs/),
you can easily set this up using the following two commands:

    rustup install nightly
    rustup target add wasm32-unknown-unknown --toolchain nightly

To actually build the game,
we currently use [`cargo web`](https://github.com/koute/cargo-web),
which you can get by running

    cargo install cargo-web

and with that done, you should be able to play the game by executing

    cargo web start --target=wasm32-unknown-unknown

and pointing your browser to [`http://localhost:8000`](http://localhost:8000).

---

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
