//! Scamper's warts format with trace objects.
//!
//! This format is typically used for [CAIDA's Ark](https://www.caida.org/projects/ark/) data.
//!
//! The [warts](https://github.com/dioptra-io/warts) library is used to read and write the
//! [`warts(5)`](https://www.caida.org/catalog/software/scamper/man/warts.5.pdf) files.
mod from_internal;
mod models;
mod reader;
mod to_internal;
mod writer;

pub use models::*;
pub use reader::*;
pub use writer::*;
