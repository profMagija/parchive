# `parchive` - a pretty archiving library

This is a simple binary reader-writer library. It allows simple definition of binary readable and writable formats.

## Using parchive

To use parchive you first need to define a struct that will hold the read data. In this example, we want to parse Java Class files. If we go to [the spec](https://docs.oracle.com/javase/specs/jvms/se17/html/jvms-4.html) we will see that the `ClassFile` is described like this (with comments added):

```c++
ClassFile {
    u4             magic;  // u4 maps to u32 in rust
    u2             minor_version;
    u2             major_version;
    u2             constant_pool_count;
    cp_info        constant_pool[constant_pool_count-1];
    u2             access_flags;
    u2             this_class;
    u2             super_class;
    u2             interfaces_count;
    u2             interfaces[interfaces_count];
    // rest omitted for brevity
}
```

So we can map this to rust syntax like so:

```rust
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
    // rest omitted for brevity
}
```

Now we can implement the `Archivable` trait on this struct:

```rust
impl Archivable for ClassFile {
    fn archive<Ar: Archive>(&mut self, ar: &mut Ar) -> Result<()> {
        // Class Files are in big-endian
        ar.set_little_endian(false);
        ar.archive(&mut self.magic)?;
        ar.archive(&mut self.minor_version)?;
        ar.archive(&mut self.major_version)?;

        // cp_count is the actual length + 1
        let mut cp_count = (self.constant_pool.len() + 1) as u16;
        ar.archive(&mut cp_count)?;
        // we will ignore the double-entries for now
        ar.archive_vec((cp_count - 1).into(), &mut self.constant_pool)?;

        ar.archive(&mut self.access_flags)?;
        ar.archive(&mut self.this_class)?;
        ar.archive(&mut self.super_class)?;
        ar.archive(&mut self.interfaces)?;
        Ok(())
    }
}
```

It seems very repetitive, but this is a small price to pay for having direct access to the archiving machinery. Lets say we want to implement the obscure java's "double constant pool entries" (all entries of type Long and Double count as two, including in the "count" field).

In a regular declarative setting, it would be complicated to implement, but here we can simply go back to "manual" implementation of the serialization/deserialization:

```rust
if Ar::IS_READING {
    let mut cp_count: u16 = 0;
    // read the count
    ar.archive(&mut cp_count)?;

    let mut i = 1;
    while i < cp_count {
        let mut v: CpInfo = Default::default();
        // read an entry
        ar.archive(&mut v)?;

        // if it is a double-entry, increment the counter by 2
        i += if v.is_double() { 2 } else { 1 };

        self.constant_pool.push(v);
    }
} else {
    // calculate the number of entries, taking the doubles into account
    let mut cp_count: u16 = self.constant_pool.iter()
        .map(|x| -> u16 {
            // each double counts as two entries
            if x.is_double() { 2 } else { 1 }
        })
        .sum();

    // write the calculated count and the entries
    ar.archive(&mut cp_count)?;
    ar.archive_vec(self.constant_pool.len(), &mut self.constant_pool)?;
}
```

Now, we need to invoke this, using either an `ArchiveReader` if we want to read , or `ArchiveWriter` if we want to write the struct:

```rust
pub fn main() {
    let file = std::fs::File::open("/path/to/java/lang/Object.class").unwrap();
    let mut ar = ArchiveReader::new(file);

    let mut cf = ClassFile::default();
    ar.archive(&mut cf).unwrap();
    println!("{:?}", cf);

    //...
}
```

### Tagged enums

Because of their prevalence, tagged enums have a special macro for easier implementation: `tagged_enum!`. The general syntax is as follows:

```rust
tagged_enum! {
    [optional pub] enum EnumName : tag_type {
        EnumCaseOne(EnumType) = tag1,
        // ...
    }
}
```

This will implement all the required parts for a `Archivable` implementation. For example:

```rust
tagged_enum! {
    enum CpInfo : u8 {
        // ...
        Integer(i32) = 3,
        Float(f32) = 4,
        Long(i64) = 5,
        // ...
    }
}
```

Check the [class example](./examples/class.rs) file for the entire implemented example.

## Why not use `nom`?

Because `nom` is geared towards parsing, while my goal was to enable both reading and writing of binary formats.

## Why not use something declarative

Declarative parsers/writers are great, but every so often you get *something* that they cannot do. For example handling in-file offsets, where the logic for reading and writing needs to differ, to compensate for changing sizes of data.

## Why not use `_`?

Because I did not know about it.