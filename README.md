rs-domino
---
Another domino game

### Status
WIP

### How to build
To build this project try this
```
$ cargo build --release --profile=pack
```

- `--release` generate `src/build.rs` (you can disable this in `build/main.rs`).
- `--profile=pack` pack all resource in one file (`resources/resource.package`).

And when these files are created, you can use `debug`/`check`/etc.
