use crate::{Archive, Result};

pub trait Archivable: Default {
    fn archive<Ar: Archive>(&mut self, ar: &mut Ar) -> Result<()>;
}

macro_rules! prim_archivable {
    ($type: ty $(, $other: ty)+) => {
        prim_archivable!($type);
        $(
            prim_archivable!($other);
        )+
    };

    ($typ: ty) => {
        impl Archivable for $typ {
            fn archive<Ar: Archive>(&mut self, ar: &mut Ar) -> Result<()> {
                if Ar::IS_READING {
                    let mut buf: [u8; std::mem::size_of::<$typ>()] = Default::default();
                    ar.read_exact(&mut buf)?;
                    *self = if ar.is_little_endian() {
                        <$typ>::from_le_bytes(buf)
                    } else {
                        <$typ>::from_be_bytes(buf)
                    }
                } else {
                    let buf = if ar.is_little_endian() {
                        self.to_le_bytes()
                    } else {
                        self.to_be_bytes()
                    };
                    ar.write_all(&buf)?;
                }
                Ok(())
            }
        }
    };
}

macro_rules! tuple_archivable {
    ($($tfs:ident $vs:ident),+) => {
        impl< $($tfs : Archivable),+ > Archivable for ( $($tfs),+ ) {
            fn archive<Ar: Archive>(&mut self, ar: &mut Ar) -> Result<()> {
                let ( $($vs),+ ) = self;
                $(
                    ar.archive($vs)?;
                )+
                Ok(())
            }
        }
    }
}

prim_archivable!(u8, u16, u32, u64, u128);
prim_archivable!(i8, i16, i32, i64, i128);
prim_archivable!(f32, f64);
prim_archivable!(usize, isize);

tuple_archivable!(T0 v0, T1 v1);
tuple_archivable!(T0 v0, T1 v1, T2 v2);
tuple_archivable!(T0 v0, T1 v1, T2 v2, T3 v3);
tuple_archivable!(T0 v0, T1 v1, T2 v2, T3 v3, T4 v4);
tuple_archivable!(T0 v0, T1 v1, T2 v2, T3 v3, T4 v4, T5 v5);
tuple_archivable!(T0 v0, T1 v1, T2 v2, T3 v3, T4 v4, T5 v5, T6 v6);
tuple_archivable!(T0 v0, T1 v1, T2 v2, T3 v3, T4 v4, T5 v5, T6 v6, T7 v7);
tuple_archivable!(T0 v0, T1 v1, T2 v2, T3 v3, T4 v4, T5 v5, T6 v6, T7 v7, T8 v8);
tuple_archivable!(T0 v0, T1 v1, T2 v2, T3 v3, T4 v4, T5 v5, T6 v6, T7 v7, T8 v8, T9 v9);
