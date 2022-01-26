use crate::Archivable;
use crate::{Archive, Result};

#[derive(Default, PartialEq, Eq)]
pub struct LenVec<TLen, T>
where
    TLen: Archivable + TryFrom<usize> + Into<usize>,
    T: Archivable,
{
    pd: std::marker::PhantomData<TLen>,
    vec: Vec<T>,
}

impl<TLen, T> Archivable for LenVec<TLen, T>
where
    TLen: Archivable + TryFrom<usize> + Into<usize>,
    T: Archivable,
{
    fn archive<Ar: Archive>(&mut self, ar: &mut Ar) -> Result<()> {
        ar.archive_len_vec::<TLen, T>(&mut self.vec)
    }
}

impl<TLen, T> std::ops::Deref for LenVec<TLen, T>
where
    TLen: Archivable + TryFrom<usize> + Into<usize>,
    T: Archivable,
{
    type Target = Vec<T>;

    fn deref(&self) -> &Vec<T> {
        &self.vec
    }
}

impl<TLen, T> std::ops::DerefMut for LenVec<TLen, T>
where
    TLen: Archivable + TryFrom<usize> + Into<usize>,
    T: Archivable,
{
    fn deref_mut(&mut self) -> &mut Vec<T> {
        &mut self.vec
    }
}

impl<TLen, T> std::fmt::Debug for LenVec<TLen, T>
where
    TLen: Archivable + TryFrom<usize> + Into<usize>,
    T: Archivable + std::fmt::Debug,
{
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.vec.fmt(fmt)
    }
}
