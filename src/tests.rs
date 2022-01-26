use super::*;
use std::io::Cursor;

#[test]
fn test_read_primitive() {
    let data: &[u8] = &[0, 1, 2, 3, 4];
    let mut ar = ArchiveReader::new(Cursor::new(data));

    let mut i: u32 = 0;
    let mut j: u8 = 0;
    ar.archive(&mut i).unwrap();
    ar.archive(&mut j).unwrap();

    assert_eq!(0x03020100, i);
    assert_eq!(0x04, j);
}

#[test]
fn test_write_primitive() {
    let mut data = Vec::<u8>::new();
    let mut ar = ArchiveWriter::new(Cursor::new(&mut data));

    let mut i: u32 = 0x03020100;
    let mut j: u8 = 0x04;
    ar.archive(&mut i).unwrap();
    ar.archive(&mut j).unwrap();

    assert_eq!(0x03020100, i);
    assert_eq!(0x04, j);
    assert_eq!(vec![0, 1, 2, 3, 4], data);
}

#[test]
fn test_read_option() {
    let data: &[u8] = &[0, 1, 2, 3];
    let mut ar = ArchiveReader::new(Cursor::new(data));

    let mut i: Option<i32> = None;
    let mut j: Option<i8> = None;
    ar.archive_option(true, &mut i).unwrap();
    ar.archive_option(false, &mut j).unwrap();

    assert_eq!(Some(0x03020100), i);
    assert_eq!(None, j);
}
#[test]
fn test_write_option() {
    let mut data = Vec::<u8>::new();
    let mut ar = ArchiveWriter::new(Cursor::new(&mut data));

    let mut i: Option<i32> = Some(0x03020100);
    let mut j: Option<i8> = None;
    ar.archive_option(true, &mut i).unwrap();
    ar.archive_option(false, &mut j).unwrap();

    assert_eq!(Some(0x03020100), i);
    assert_eq!(None, j);
    assert_eq!(vec![0, 1, 2, 3], data);
}

#[test]
fn test_read_vec() {
    let data: &[u8] = &[0, 0, 0, 0, 1, 0, 0, 0, 2, 0, 0, 0];
    let mut ar = ArchiveReader::new(Cursor::new(data));

    let mut i: Vec<i32> = vec![];
    ar.archive_vec(3, &mut i).unwrap();

    assert_eq!(vec![0, 1, 2], i);
}

#[test]
fn test_write_vec() {
    let mut data = Vec::<u8>::new();
    let mut ar = ArchiveWriter::new(Cursor::new(&mut data));

    let mut i: Vec<i32> = vec![0, 1, 2];
    ar.archive_vec(3, &mut i).unwrap();

    assert_eq!(vec![0, 1, 2], i);
    assert_eq!(vec![0, 0, 0, 0, 1, 0, 0, 0, 2, 0, 0, 0], data);
}

#[derive(Default, Debug, PartialEq, Eq)]
struct Custom {
    field_1: i8,
    field_2: i32,
}

impl Archivable for Custom {
    fn archive<Ar: Archive>(&mut self, ar: &mut Ar) -> Result<()> {
        ar.archive(&mut self.field_1)?;
        ar.archive(&mut self.field_2)?;
        Ok(())
    }
}

#[test]
fn test_read_custom() {
    let data: &[u8] = &[1, 2, 3, 4, 5];
    let mut ar = ArchiveReader::new(Cursor::new(data));

    let mut value = Custom::default();
    ar.archive(&mut value).unwrap();

    assert_eq!(
        Custom {
            field_1: 1,
            field_2: 0x05040302
        },
        value
    );
}

#[test]
fn test_write_custom() {
    let mut data = Vec::<u8>::new();
    let mut ar = ArchiveWriter::new(Cursor::new(&mut data));

    let mut value = Custom {
        field_1: 1,
        field_2: 0x05040302,
    };
    ar.archive(&mut value).unwrap();

    assert_eq!(
        Custom {
            field_1: 1,
            field_2: 0x05040302
        },
        value
    );
    assert_eq!(vec![1, 2, 3, 4, 5], data);
}

#[test]
fn test_read_len_vec() {
    let data: &[u8] = &[2, 2, 3, 4, 5];
    let mut ar = ArchiveReader::new(Cursor::new(data));

    let mut value = LenVec::<u8, u16>::default();
    ar.archive(&mut value).unwrap();

    assert_eq!(vec![0x0302, 0x0504], *value);
}

#[test]
fn test_write_len_vec() {
    let mut data = Vec::<u8>::new();
    let mut ar = ArchiveWriter::new(Cursor::new(&mut data));

    let mut value = LenVec::<u8, u16>::new(vec![0x0302, 0x0504]);
    ar.archive(&mut value).unwrap();

    assert_eq!(vec![0x0302, 0x0504], *value);
    assert_eq!(vec![2, 2, 3, 4, 5], data);
}
