//! PArchive is a binary reader/writer library based aroungdthe concept of `Archive`s.
//!
//! The main goal is to allow implementing a binary reader and writer for a given type,
//! with as little as possible code duplication between them. It is based on the following
//! observation: In the reader code, one would have statements in the form of
//!
//! ```ignore
//! self.field = reader.read()?;
//! ```
//!
//! and in the writer, statements of the form
//!
//! ```ignore
//! writer.write(self.field)?;
//! ```
//!
//! Around them, a lot of the code will be repeated, simply because reading a structure is
//! very similar to writing it, e.g.:
//!
//! ```ignore
//! // in reader
//! self.is_present = reader.read()?;
//! if self.is_present {
//!     self.something = reader.read()?;
//! }
//!
//! // in writer
//! writer.write(self.is_present)?;
//! if self.is_present {
//!     writer.write(self.something)?;
//! }
//! ```
//!
//! If we observe that both `read` and `write` can be merged into a single method `archive`,
//! which takes a mutable reference and either modifies it in-place (for reading from file),
//! or just reads from it (for writing to file). Then the two pieces of code can be merged:
//!
//! ```ignore
//! ar.archive(&mut self.is_present)?;
//! if self.is_present {
//!     ar.archive(&mut self.something)?;
//! }
//! ```
//!
//! This allows us to combine the reader and writer code. And for cases where we still need to
//! differentiate between the two, we still can, using a simple `if`.
//!
//! ```ignore
//! if Ar::IS_READING {
//!     // reader-only code
//! } else {
//!     // writer-only code
//! }
//! ```

#[macro_use]
mod macros;

mod archivable;
mod archive;
mod len_string;
mod len_vec;
mod reader;
mod result;
mod writer;

pub use archivable::Archivable;
pub(crate) use archive::ArchiveInternal;
pub use archive::{Archive, ArchiveSeekable};
pub use len_vec::LenVec;
pub use reader::ArchiveReader;
pub use result::{Error, Result};
pub use writer::ArchiveWriter;

#[cfg(test)]
mod tests;
