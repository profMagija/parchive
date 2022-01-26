use crate::result::io_error;
use crate::{Archivable, Archive, ArchiveInternal, ArchiveSeekable, Result};
use std::io::{Read, Seek};

/// A reader part of the Archive mechanism.
pub struct ArchiveReader<R: Read> {
    read: R,
    little_endian: bool,
}

impl<R: Read> ArchiveReader<R> {
    pub fn new(read: R) -> Self {
        Self {
            read,
            little_endian: true,
        }
    }
}

impl<R: Read> ArchiveInternal for ArchiveReader<R> {
    fn write_all(&mut self, _: &[u8]) -> Result<()> {
        unreachable!()
    }

    fn read_exact(&mut self, value: &mut [u8]) -> Result<()> {
        self.read.read_exact(value).or_else(io_error)
    }
}

impl<R: Read> Archive for ArchiveReader<R> {
    const IS_READING: bool = true;

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

impl<R: Read + Seek> ArchiveSeekable for ArchiveReader<R> {
    fn seek(&mut self, from: std::io::SeekFrom) -> Result<u64> {
        self.read.seek(from).or_else(io_error)
    }
}
