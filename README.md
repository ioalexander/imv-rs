# imv-rs - simple image viewer

imv-rs is a small, no-bullshit image viewer written in Rust using `egui` and `image`.

This whole thing started when the original [imv](https://sr.ht/~exec64/imv/) began breaking apart - dependencies got old, stuff wouldn’t compile, and Arch dropped FreeImage. Rebuilding it turned into a pain, so I just made my own.

## Install

### AUR

```
paru -S imv-rs-bin
```

```
yay -S imv-rs-bin
```

### Source

```
git clone https://github.com/ioalexander/imv-rs
cd imv-rs
cargo build --release
```

## Usage

`H` or `←` - Go to previous file
`L` or `→` - Go to next file

### What works out of the box

- PNG
- JPG/JPEG
- WEBP
- GIF

### Roadmap

- Animated WEBP
- Animated PNG
- Performance improvements for image decoding / async loading with status

No weird library setup, no fighting with ancient dependencies. Just build and run.

### Why bother?

The original imv is great when it works - but getting it to build can feel like archaeology. imv-rs skips all that, thanks to Rust and it's portable binaries.
