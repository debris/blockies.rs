# blockies.rs

[![Build Status][travis-image]][travis-url]

[travis-image]: https://travis-ci.org/debris/blockies.rs.svg?branch=master
[travis-url]: https://travis-ci.org/debris/blockies.rs

library that generates blocky identicons

Rust implementation of javascript [blockies](https://github.com/download13/blockies) library. Supports also [ethereum blockies](https://github.com/alexvandesande/blockies)

### Library usage

```rust
use blockies::Ethereum;

let blockies = Ethereum::default();
let mut png = Vec::new();

blockies.create_icon(&mut png, b"0x01122df2b7d1c0a6ad94589da045af3885bedbbc");

// `png` now contains a rendered image of the blockies for that address
assert_eq!(png.len(), 179);
```

### CLI Usage

build with

```
cargo build --release -p blockies-cli
```

```
blockies 0.2.1
debris <marek.kotewicz@gmail.com>
blockies cli

USAGE:
    blockies [OPTIONS] --seed <seed>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --mode <mode>        Choose blockies image type
        --output <output>    Output file path
        --scale <scale>      Blockies pixel size
        --seed <seed>        Seed phrase for generating new image
        --size <size>        Blockies image size
```

### TODO

- specifying colors via cli
