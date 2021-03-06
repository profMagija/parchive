extern crate parchive;

use parchive::{tagged_enum, Archivable, Archive, ArchiveReader, ArchiveWriter, LenVec, Result};

tagged_enum! {
    enum CpInfo : u8 {
        Utf8(LenVec<u16, u8>) = 1,
        Integer(i32) = 3,
        Float(f32) = 4,
        Long(i64) = 5,
        Double(f64) = 6,
        Class(u16) = 7,
        String(u16) = 8,
        Fieldref((u16, u16)) = 9,
        Methodref((u16, u16)) = 10,
        InterfaceMethodref((u16, u16)) = 11,
        NameAndType((u16, u16)) = 12,
        MethodHandle((u8, u16)) = 15,
        MethodType(u16) = 16,
        Dynamic((u16, u16)) = 17,
        InvokeDynamic((u16, u16)) = 18,
        Module(u16) = 19,
        Package(u16) = 20
    }
}

impl CpInfo {
    fn is_double(&self) -> bool {
        matches!(self, Self::Long(_) | Self::Double(_))
    }
}

#[derive(Default, Debug)]
struct ClassFile {
    magic: u32,
    minor_version: u16,
    major_version: u16,
    // we will not store the length directly
    constant_pool: Vec<CpInfo>,
    access_flags: u16,
    this_class: u16,
    super_class: u16,
    // a length-prefixed vector, with a `u16` length and `u16` values.
    interfaces: LenVec<u16, u16>,
}

impl Archivable for ClassFile {
    fn archive<Ar: Archive>(&mut self, ar: &mut Ar) -> Result<()> {
        // Class Files are in big-endian
        ar.set_little_endian(false);
        ar.archive(&mut self.magic)?;
        ar.archive(&mut self.minor_version)?;
        ar.archive(&mut self.major_version)?;

        if Ar::IS_READING {
            let mut cp_count: u16 = 0;
            ar.archive(&mut cp_count)?;

            let mut i = 1;
            while i < cp_count {
                let mut v: CpInfo = Default::default();
                ar.archive(&mut v)?;
                i += if v.is_double() { 2 } else { 1 };
                self.constant_pool.push(v);
            }
        } else {
            let mut cp_count: u16 = self
                .constant_pool
                .iter()
                .map(|x| -> u16 {
                    if x.is_double() {
                        2
                    } else {
                        1
                    }
                })
                .sum();
            ar.archive(&mut cp_count)?;
            ar.archive_vec(self.constant_pool.len(), &mut self.constant_pool)?;
        }

        ar.archive(&mut self.access_flags)?;
        ar.archive(&mut self.this_class)?;
        ar.archive(&mut self.super_class)?;
        ar.archive(&mut self.interfaces)?;
        Ok(())
    }
}

pub fn main() {
    let file = std::fs::File::open("./test/Object.class").unwrap();
    let mut ar = ArchiveReader::new(file);

    let mut cf = ClassFile::default();
    ar.archive(&mut cf).unwrap();
    println!("{:?}", cf);

    cf.major_version = 50;
    let file2 = std::fs::File::create("./test/Object2.class").unwrap();
    let mut ar2 = ArchiveWriter::new(file2);
    ar2.archive(&mut cf).unwrap();
}
