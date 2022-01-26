#[macro_use]
mod macros;

mod archive;
mod archivable;
mod len_vec;
mod reader;
mod result;
mod writer;

pub use archivable::Archivable;
pub use len_vec::LenVec;
pub use reader::ArchiveReader;
pub use result::{Error, Result};
pub use writer::ArchiveWriter;
pub use archive::{Archive, ArchiveSeekable};
pub(crate) use archive::ArchiveInternal;

#[cfg(test)]
mod tests;
