use crate::Archivable;
use crate::Error;
use crate::LenVec;
use crate::{Archive, Result};
use std::marker::PhantomData;

#[derive(Default)]
pub struct LenStringUtf8<TLen>
where
    TLen: Archivable + TryFrom<usize> + Into<usize>,
{
    pd: PhantomData<TLen>,
    string: String,
}

impl<TLen> LenStringUtf8<TLen>
where
    TLen: Archivable + TryFrom<usize> + Into<usize>,
{
    fn encode(&self) -> Vec<u8> {
        self.string.clone().into_bytes()
    }

    fn decode(&mut self, vec: Vec<u8>) -> Result<()> {
        self.string = String::from_utf8(vec)
            .or_else(|e| Err(Error::ValueError(format!("invalid utf-8: {}", e))))?;
        Ok(())
    }
}

impl<TLen> Archivable for LenStringUtf8<TLen>
where
    TLen: Archivable + TryFrom<usize> + Into<usize>,
{
    fn archive<Ar: Archive>(&mut self, ar: &mut Ar) -> Result<()> {
        let mut lv = LenVec::<TLen, _>::new(if Ar::IS_READING {
            vec![]
        } else {
            self.encode()
        });
        ar.archive(&mut lv)?;
        self.decode(lv.into())
    }
}
