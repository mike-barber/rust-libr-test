# Basic example calling into R from Rust

This uses two crates maintained by the [extendr team](https://github.com/extendr):
- [libR-sys](https://github.com/extendr/libR-sys) used to start an R environment in the current Rust process
- [extendr](https://github.com/extendr/extendr) used to interface with the R environment; normally this crate is used to permit R to call into Rust, and allow the Rust functions to call back into R. We're using it just to call into R directly, following the example of how the `extendr` team run R in order to perform their tests.

The `extendr_api` makes it pretty simple to call into R and evaluate results.


# Linux

Just have R installed essentially. Set `R_HOME` if it's in a non-standard location.

# Windows-specific instructions

It does work on Windows, but requires a bit more work. We need to target the `x86_64-pc-windows-gnu` toolchain because this is how R is linked.

We also need Rtools40 installed, and the following environment variables set:
- `R_HOME=C:\R` or wherever your R install is
- On `PATH`, append `C:\rtools40\mingw64\bin;C:\R\bin\x64;` as you need the mingw64 gnu linker etc.

It's easiest to set the default toolchain to GNU for this project directory thus:
```
rustup override set stable-x86_64-pc-windows-gnu
```

Then it's possible to build and run with the usual:
```
cargo build
cargo run
```

Alternatively, you can build and run with the target specified explicitly:
```
cargo build --target x86_64-pc-windows-gnu
cargo run --target x86_64-pc-windows-gnu
```


