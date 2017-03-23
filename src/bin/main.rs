#[cfg(feature = "cli")]
include!("blockies.rs");

#[cfg(not(feature = "cli"))]
fn main() {}
