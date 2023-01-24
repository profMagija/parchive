// #[macro_export]
// macro_rules! ar {
//     (priv $ar: expr => { >> ? [ $present:expr ] = $tgt:expr ; $($rest:tt)* }) => {
//         $ar.archive_option($present, &mut $tgt)?;
//         ar!(priv $ar => { $($rest)* });
//     };

//     (priv $ar: expr => { >> * [ $length:expr ] = $tgt:expr ; $($rest:tt)* }) => {
//         $ar.archive_vec($length, &mut $tgt)?;
//         ar!(priv $ar => { $($rest)* });
//     };

//     (priv $ar: expr => { >> $tgt:expr ; $($rest:tt)* }) => {
//         $ar.archive(&mut $tgt)?;
//         ar!(priv $ar => { $($rest)* });
//     };

//     (priv $ar: expr => { $tgt:stmt ; $($rest:tt)* }) => {
//         $tgt
//         ar!(priv $ar => { $($rest)* });
//     };

//     (priv $ar: expr => { ; }) => {};
//     (priv $ar: expr => { }) => {};

//     ($ar: expr => $($rest:tt)*) => {
//         || -> $crate::Result<()> {
//             ar!(priv $ar => $($rest)*);
//             Ok(())
//         }()
//     };
// }

#[macro_export]
macro_rules! tagged_enum {
    ($mods:vis enum $name:ident : $tagTy:ty {
        $($cName:ident($cTy:ty) = $cTag:literal),*
    }) => {
        use $crate::*;
        #[derive(Debug)]
        $mods enum $name {
            $(
                $cName($cTy),
            )*
            invalid
        }

        impl $name {
            fn tag(&self) -> $tagTy {
                match self {
                    $(
                        Self::$cName(_) => $cTag,
                    )*
                    Self::invalid => 0,
                }
            }
        }
        impl Default for $name {
            fn default() -> Self { Self::invalid }
        }

        impl Archivable for $name {
            fn archive<Ar: Archive>(&mut self, ar: &mut Ar) -> Result<()> {
                let mut tag = self.tag();
                ar.archive(&mut tag)?;

                if Ar::IS_READING {
                    match tag {
                        $(
                            $cTag => {
                                let mut v = Default::default();
                                ar.archive(&mut v)?;
                                *self = Self::$cName(v);
                                Ok(())
                            }
                        )*
                        i => Err(Error::ValueError(format!("invalid tag for enum {:x}", i)))
                    }
                } else {
                    match self {
                        $(
                            Self::$cName(v) => {
                                ar.archive(v)?;
                                Ok(())
                            }
                        )*
                        Self::invalid => Err(Error::ValueError("writing invalid enum value".to_string()))
                    }
                }
            }
        }
    };
}
