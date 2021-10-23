# Basic example calling into R from Rust

This uses two crates maintained by the [extendr team](https://github.com/extendr):
- [libR-sys](https://github.com/extendr/libR-sys) used to start an R environment in the current Rust process
- [extendr](https://github.com/extendr/extendr) used to interface with the R environment; normally this crate is used to permit R to call into Rust, and allow the Rust functions to call back into R. We're using it just to call into R directly, following the example of how the `extendr` team run R in order to perform their tests.

The `extendr_api` makes it pretty simple to call into R and evaluate results.
