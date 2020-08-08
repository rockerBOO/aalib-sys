# aalib bindings for Rust

Automated bindings for the [aalib library](http://aa-project.sourceforge.net/aalib/).

This is currently not tested in terms of it working. If you are looking to make safe bindings you could use this to build your bindings to start with.

I am interested in image/video to ascii conversion, but libcaca (another image to ascii library) provides simular results. Libcaca already has safe bindings for Rust `[caca-rs](https://github.com/Ruin0x11/caca-rs)`.

## Setup

Install bindgen.

```sh-script
cargo install bindgen
```

Download the aalib source code.

```
http://aa-project.sourceforge.net/aalib/
```

## Build

Using bindgen to make the src/bindings using the aalib `aalib.h`. Only including `aa_` prefixed functions.

```sh-script
bindgen ../aalib/src/aalib.h -o src/bindings.rs  --whitelist-function '^aa_.*' --whitelist-var '^mem_d$' --whitelist-var '^save_d$' --whitelist-var '^aa_.*'
```

### Links:

- [caca-sys](https://github.com/Ruin0x11/caca-sys/)
- [libcaca](http://caca.zoy.org/wiki/libcaca)
- [Using C Libraries in Rust](https://medium.com/dwelo-r-d/using-c-libraries-in-rust-13961948c72a)
