//! Scamper's warts format with trace objects.
pub mod from;
pub mod reader;
pub mod to;
pub mod writer;

pub use reader::WartsTraceReader;
pub use writer::WartsTraceWriter;
