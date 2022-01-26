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
    fn archive<Ar: Archive>(&mut self, ar: &mut Ar) {
        // Class Files are in big-endian
        ar.set_little_endian(false);

        ar.archive(&mut self.magic);
        ar.archive(&mut self.minor_version);
        ar.archive(&mut self.major_version);

        // cp_count is the actual length + 1
        let mut cp_count = (self.constant_pool.len() + 1) as u16;
        ar.archive(&mut cp_count);
        ar.archive_vec((cp_count - 1).into(), &mut self.constant_pool);

        ar.archive(&mut self.access_flags);
        ar.archive(&mut self.this_class);
        ar.archive(&mut self.super_class);
        ar.archive(&mut self.interfaces);
    }
}
```

It seems very repetitive, but this is a small price to pay for having direct access to the 

## Why not use `nom`?

Because `nom` is geared towards parsing, while my goal was to enable both reading and writing of binary formats.

## Why not use something declarative

Declarative parsers/writers are great, but every so often you get *something* that they cannot do. For example handling in-file offsets, where the logic for reading and writing needs to differ, to compensate for changing sizes of data.

## Why not use `_`?

Because I did not know about it.