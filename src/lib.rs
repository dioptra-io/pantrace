//! Pantrace converts between traceroute formats, in the same way as [Pandoc](https://pandoc.org) converts between document formats.
//!
//! For more information please refer to the [GitHub repository](https://github.com/dioptra-io/pantrace).
#![feature(ip)]
#![feature(slice_group_by)]
#![feature(trait_alias)]

pub mod atlas;
pub mod internal;
pub mod iris;
pub mod traits;
pub mod utils;
pub mod warts_trace;
