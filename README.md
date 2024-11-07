This repository contains resources for my talk "Chips don't bite!", that I will be giving at RustLab 2024.
https://rustlab.it/talks/chips-don-t-bite-a-fearless-introduction-to-ucontroller-programming-for-rusteacea

[Slides (PDF)](slides.pdf)

# Resources
If you want to get started with embedded Rust I can recommend these resources:
* [micro::bit v2 Embedded Discovery Book](https://github.com/rust-embedded/discovery-mb2)
  * An introductory book
  * Currently being worked on -> please report any problems
* [Rust Embedded Matrix Channel](https://matrix.to/#/#rust-embedded:matrix.org)
  * A matrix channel with a lot of friendly people
* [`drive-rs` Driver Listing](https://tweedegolf.github.io/drive-rs/)
  * A list of drivers and chips you can use with embedded Rust
  * My side project
* [YouTube: Rusty Bits](https://www.youtube.com/@therustybits)
  * [From Zero to Async in Embedded Rust](https://youtu.be/wni5h5vIPhU)

# How to contact me
* [@tamme@fosstodon.org](https://fosstodon.org/@tamme)
* E-Mail: [tamme@tweedegolf.com](mailto:tamme@tweedegolf.com)
* [Blog](https://tweedegolf.nl/en/about/30/tamme)
* [LinkedIn](https://www.linkedin.com/in/tamme-dittrich-81b225227/)

# (Rough) Steps how I got to this example
1. Find a board support crate (BSP) for my [Metro RP2040](https://www.adafruit.com/product/5786) board: [`adafruit-metro-rp2040`](https://crates.io/crates/adafruit-metro-rp2040)
2. `cargo new metro-co2`
3. Copy the `adafruit_metro_rainbow` example into my `main.rs`
4. Copy the [`.cargo/config.toml`](https://github.com/rp-rs/rp-hal-boards/blob/main/.cargo/config.toml) from the `rp-hal-baords` repository root
5. Try to build, add missing dependencies from the BSPs `Cargo.toml`
6. Run and see the LED changing color
7. `cargo add scd4x`
8. Copy over the relevant parts of the `scd4x` example into my `main.rs`
9. Copy I2C initialization from [`rp-hal-examples`](https://github.com/rp-rs/rp-hal/blob/main/rp2040-hal-examples/src/bin/i2c.rs#L87-L94)
10. Write some glue to turn the CO2 measurement into a color
