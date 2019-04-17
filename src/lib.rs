//! # blockies.rs
//!
//! library that generates blocky identicons
//!
//! Rust implementation of javascript [blockies](https://github.com/download13/blockies) library. Supports also [ethereum blockies](https://github.com/alexvandesande/blockies)
//!
//! ### Library usage
//!
//! ```rust
//! use blockies::Ethereum;
//!
//! let blockies = Ethereum::default();
//! let mut png = Vec::new();
//!
//! blockies.create_icon(&mut png, b"0x01122df2b7d1c0a6ad94589da045af3885bedbbc");
//!
//! // `png` now contains a rendered image of the blockies for that address
//! assert_eq!(png.len(), 179);
//! ```

mod classic;
mod ethereum;
pub(crate) mod util;

pub use classic::Classic;
pub use ethereum::Ethereum;
pub use pixelate::Error;
