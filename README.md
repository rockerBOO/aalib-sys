# aalib bindings for Rust

Automated bindings for the [aalib library](http://aa-project.sourceforge.net/aalib/).

aalib-sys

# Build

```sh-script
bindgen .../aalib-1.4.0/src/aalib.h -o src/bindings.rs  --whitelist-function '^aa_.*'
```
