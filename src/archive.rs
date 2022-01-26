use crate::{Archivable, Result};

pub trait ArchiveInternal {
    fn write_all(&mut self, value: &[u8]) -> Result<()>;
    fn read_exact(&mut self, value: &mut [u8]) -> Result<()>;
}

pub trait Archive: ArchiveInternal + Sized {
    const IS_READING: bool;

    fn archive(&mut self, value: &mut impl Archivable) -> Result<()>;

    fn set_little_endian(&mut self, little_endian: bool);
    fn is_little_endian(&self) -> bool;

    fn archive_option<T: Archivable>(
        &mut self,
        is_present: bool,
        value: &mut Option<T>,
    ) -> Result<()> {
        if Self::IS_READING {
            if is_present {
                let mut um = T::default();
                self.archive(&mut um)?;
                *value = Some(um);
            } else {
                *value = None;
            }
        } else {
            assert_eq!(is_present, value.is_some());
            if is_present {
                self.archive(value.as_mut().unwrap())?;
            }
        }
        Ok(())
    }

    fn archive_vec<T: Archivable>(&mut self, length: usize, value: &mut Vec<T>) -> Result<()> {
        if Self::IS_READING {
            *value = Vec::with_capacity(length);
            for _ in 0..length {
                let mut um = T::default();
                self.archive(&mut um)?;
                value.push(um);
            }
            Ok(())
        } else {
            assert_eq!(length, value.len());
            for v in value.iter_mut() {
                self.archive(v)?;
            }
            Ok(())
        }
    }

    fn archive_len_vec<TLen: Archivable + Into<usize> + TryFrom<usize>, T: Archivable>(
        &mut self,
        value: &mut Vec<T>,
    ) -> Result<()> {
        let len = TLen::try_from(value.len());
        if let Ok(mut len) = len {
            self.archive(&mut len)?;
            self.archive_vec(len.into(), value)?;
            Ok(())
        } else {
            panic!("invalid length")
        }
    }

    fn archive_slice<T: Archivable>(&mut self, length: usize, value: &mut [T]) -> Result<()> {
        for i in 0..length {
            self.archive(&mut value[i])?;
        }
        Ok(())
    }
}

pub trait ArchiveSeekable: Archive {
    fn seek(&mut self, from: std::io::SeekFrom) -> Result<u64>;
}


