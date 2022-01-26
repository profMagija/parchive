use crate::result::io_error;
use crate::{Archivable, Archive, ArchiveInternal, ArchiveSeekable, Result};
use std::io::{Seek, Write};

pub struct ArchiveWriter<W: Write> {
    write: W,
    little_endian: bool,
}

impl<W: Write> ArchiveWriter<W> {
    pub fn new(write: W) -> Self {
        Self {
            write,
            little_endian: true,
        }
    }
}

impl<R: Write> ArchiveInternal for ArchiveWriter<R> {
    fn write_all(&mut self, value: &[u8]) -> Result<()> {
        self.write.write_all(value).or_else(io_error)
    }

    fn read_exact(&mut self, _: &mut [u8]) -> Result<()> {
        unreachable!()
    }
}

impl<W: Write> Archive for ArchiveWriter<W> {
    const IS_READING: bool = false;

    fn archive(&mut self, value: &mut impl Archivable) -> Result<()> {
        value.archive(self)
    }

    fn set_little_endian(&mut self, little_endian: bool) {
        self.little_endian = little_endian;
    }

    fn is_little_endian(&self) -> bool {
        self.little_endian
    }
}

impl<W: Write + Seek> ArchiveSeekable for ArchiveWriter<W> {
    fn seek(&mut self, from: std::io::SeekFrom) -> Result<u64> {
        self.write.seek(from).or_else(io_error)
    }
}
