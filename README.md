# aalib bindings for Rust

Automated bindings for the [aalib library](http://aa-project.sourceforge.net/aalib/).

## Build

```sh-script
bindgen .../aalib-1.4.0/src/aalib.h -o src/bindings.rs  --whitelist-function '^aa_.*'
```

### Links:

- [Using C Libraries in Rust](https://medium.com/dwelo-r-d/using-c-libraries-in-rust-13961948c72a)
