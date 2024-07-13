//! Miscellaneous code.

use serde::Serialize;
use std::path::Path;

/// Utility to plot with matplotlib
pub fn write_pickle(x: impl Serialize, f: impl AsRef<Path>) {
    let mut file = std::fs::File::create(f).unwrap();
    serde_pickle::to_writer(&mut file, &x, Default::default()).unwrap();
}
